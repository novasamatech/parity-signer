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
    var screen: SignerScreen
    var screenLabel: String
    var back: Bool
    var footer: Bool
    var footerButton: String
    var rightButton: String
    var screenNameType: String
    var modal: SignerModal
    var alert: SignerAlert
    
    //TODO: maybe replace explicits with rust call
    init() {
        screen = .Log
        screenLabel = "Log"
        back = false
        footer = true
        footerButton = "Log"
        rightButton = ""
        screenNameType = "h1"
        modal = .Empty
        alert = .Empty
    }
}

/**
 * Struct to store main navstate of the screen
 */
enum SignerScreen: String, Decodable {
    case Scan
    case Keys
    case Settings
    case Log
    case LogDetails
    case Transaction
    case SeedSelector
    case KeyDetails
    case Backup
    case NewSeed
    case RecoverSeedName
    case RecoverSeedPhrase
    case DeriveKey
    case Verifier
    case ManageNetwork
}

/**
 * State of transaction progress - flow starts on successful scan
 */
enum TransactionState: Equatable {
    case none
    case parsing
    case preview
    case password
    case signed
}

/**
 * Modals shown in key management screen
 */
enum SignerModal: String, Equatable, Decodable {
    case Empty
    case Error
    case newKey
    case showKey
    case seedBackup
    case keyDeleteConfirm
    case seedSelector
    case networkManager
    case networkDetails
    case NewSeedMenu
}

enum SignerAlert: String, Equatable, Decodable {
    case Empty
    case Error
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
    case NewtorkSelector
}

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    func pushButton(buttonID: ButtonID, details: String = "") {
        print(buttonID)
        //Poor man's mutex; just because it's really managed by UI abstraction
        if actionAvailable {
            /** No returns below or things will stall! **/
            actionAvailable = false
            var err = ExternError()
            withUnsafeMutablePointer(to: &err) {err_ptr in
                let res = act(err_ptr, String(describing: buttonID), details)
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
