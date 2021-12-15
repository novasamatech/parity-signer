//
//  Navigator.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

//This is a custom navigator to keep this code somewhat similar to what android has
//and implement some simple shallow navigation without pulling legacy or experimental libs

import Foundation

struct ActionResult: Decodable {
    var screen: SignerScreen = .Log(MLog())
    var screenLabel: String = ""
    var back: Bool = false
    var footer: Bool = true
    var footerButton: String = ""
    var rightButton: String = ""
    var screenNameType: String = ""
    var modal: SignerModal = .Empty
    var alert: SignerAlert = .Empty
    
    enum CodingKeys: String, CodingKey {
        case screen
        case screenLabel
        case back
        case footer
        case footerButton
        case rightButton
        case screenNameType
        case modal
        case alert
        case screenData
        case modalData
        case alertData
    }
    
    init() {}
    
    //TODO: maybe replace explicits with rust call
    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)
        screenLabel = try values.decode(String.self, forKey: .screenLabel)
        back = try values.decode(Bool.self, forKey: .back)
        footer = try values.decode(Bool.self, forKey: .footer)
        footerButton = try values.decode(String.self, forKey: .footerButton)
        rightButton = try values.decode(String.self, forKey: .rightButton)
        screenNameType = try values.decode(String.self, forKey: .screenNameType)
        let alertType = try values.decode(String.self, forKey: .alert)
        let modalType = try values.decode(String.self, forKey: .modal)
        let screenType = try values.decode(String.self, forKey: .screen)
        
        switch screenType {
        case "Scan":
            screen = .Scan
        case "Keys":
            screen = .Keys(try values.decode(MKeys.self, forKey: .screenData))
        case "Settings":
            screen = .Settings
        case "Log":
            screen = .Log(try values.decode(MLog.self, forKey: .screenData))
        case "LogDetails":
            screen = .LogDetails
        case "SeedSelector":
            screen = .SeedSelector(try values.decode(MSeeds.self, forKey: .screenData))
        case "KeyDetails":
            screen = .KeyDetails(try values.decode(MKeyDetails.self, forKey: .screenData))
        case "Backup":
            screen = .Backup
        case "NewSeed":
            screen = .NewSeed
        case "RecoverSeedName":
            screen = .RecoverSeedName
        case "RecoverSeedPhrase":
            screen = .RecoverSeedPhrase
        case "Transaction":
            screen = .Transaction(try values.decode(MTransaction.self, forKey: .screenData))
        case "DeriveKey":
            screen = .DeriveKey(try values.decode(MDeriveKey.self, forKey: .screenData))
        case "Verifier":
            screen = .Verifier
        case "ManageNetwork":
            screen = .ManageNetwork
        default:
            screen = .Log(MLog())
        }
        
        switch modalType {
        case "Empty":
            modal = .Empty
        case "NewSeedMenu":
            modal = .NewSeedMenu
        case "NetworkSelector":
            modal = .NetworkMenu(try values.decode(MNetworkMenu.self, forKey: .modalData))
        case "SeedMenu":
            modal = .SeedMenu
        case "Backup":
            modal = .Backup(try values.decode(MBackup.self, forKey: .modalData))
        case "PasswordConfirm":
            modal = .PasswordConfirm(try values.decode(MPasswordConfirm.self, forKey: .modalData))
        case "EnterPassword":
            modal = .EnterPassword(try values.decode(MEnterPassword.self, forKey: .modalData))
        case "SignatureReady":
            modal = .SignatureReady(try values.decode(MSignatureReady.self, forKey: .modalData))
        default:
            modal = .Empty
        }
        
        switch alertType {
        case "Empty":
            alert = .Empty
        case "Error":
            alert = .Error(try values.decode(MError.self, forKey: .alertData))
        default:
            alert = .Empty
        }
    }
}

/**
 * Struct to store main navstate of the screen
 */
enum SignerScreen: Decodable {
    case Scan
    case Keys(MKeys)
    case Settings
    case Log(MLog)
    case LogDetails
    case Transaction(MTransaction)
    case SeedSelector(MSeeds)
    case KeyDetails(MKeyDetails)
    case Backup
    case NewSeed
    case RecoverSeedName
    case RecoverSeedPhrase
    case DeriveKey(MDeriveKey)
    case Verifier
    case ManageNetwork
}

/**
 * Modals shown in key management screen
 */
enum SignerModal: Decodable {
    case Empty
    case NewSeedMenu
    case NetworkMenu(MNetworkMenu)
    case SeedMenu
    case Backup(MBackup)
    case PasswordConfirm(MPasswordConfirm)
    case SignatureReady(MSignatureReady)
    case EnterPassword(MEnterPassword)
}

enum SignerAlert: Decodable {
    case Empty
    case Error(MError)
    case Shield
    case keyDeleteConfirm
}

enum ButtonID {
    case Start
    case NavbarLog
    case NavbarScan
    case NavbarKeys
    case NavbarSettings
    case GoBack
    case SelectSeed
    case RightButton
    case Shield
    case SelectKey
    case GoForward
    case Derive
    case Delete
    case NewSeed
    case RecoverSeed
    case NetworkSelector
    case NextUnit
    case PreviousUnit
    case NewKey
    case BackupSeed
    case CheckPassword
    case ChangeNetwork
    case TransactionFetched
}

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    func pushButton(buttonID: ButtonID, details: String = "", seedPhrase: String = "") {
        print(buttonID)
        //Poor man's mutex; just because it's really managed by UI abstraction
        if actionAvailable {
            /** No returns below or things will stall! **/
            actionAvailable = false
            var err = ExternError()
            withUnsafeMutablePointer(to: &err) {err_ptr in
                let res = act(err_ptr, String(describing: buttonID), details, seedPhrase)
                if (err_ptr.pointee.code == 0) {
                    print(String(cString: res!))
                    if let actionResultJSON = String(cString: res!).data(using: .utf8) {
                        print(actionResultJSON)
                        if let newActionResult = try? JSONDecoder().decode(ActionResult.self, from: actionResultJSON)
                        {
                            print(newActionResult)
                            actionResult = newActionResult
                        } else {
                            print("bushing button failed on decoding!")
                        }
                    }
                    signer_destroy_string(res!)
                } else {
                    print("pushing button failed")
                }
            }
            //Boink! debounce is here
            Timer.scheduledTimer(withTimeInterval: debounceTime, repeats: false, block: {_ in self.actionAvailable = true})
            /** Return is allowed again **/
        }
    }
}
