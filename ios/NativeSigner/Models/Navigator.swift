//
//  Navigator.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

//This is a custom navigator to keep this code somewhat similar to what android has
//and implement some simple shallow navigation without pulling legacy or experimental libs

import Foundation


/**
 * Struct to store main navstate of the screen
 */
enum SignerScreen {
    case Scan
    case Keys(MKeys)
    case Settings(MSettings)
    case Log(MLog)
    case LogDetails(MLogDetails)
    case Transaction(MTransaction)
    case SeedSelector(MSeeds)
    case KeyDetails(MKeyDetails)
    case NewSeed(MNewSeed)
    case RecoverSeedName(MRecoverSeedName)
    case RecoverSeedPhrase(MRecoverSeedPhrase)
    case DeriveKey(MDeriveKey)
    case Verifier(MVerifierDetails)
    case ManageNetworks(MManageNetworks)
    case NetworkDetails(MNetworkDetails)
    case SignSufficientCrypto(MSignSufficientCrypto)
    case SelectSeedForBackup(MSeeds)
    case Documents
    case KeyDetailsMulti(MKeyDetailsMulti)
}

/**
 * Modals shown in key management screen
 */
enum SignerModal: Decodable {
    case Empty
    case NewSeedMenu
    case NetworkMenu(MNetworkMenu)
    case SeedMenu(MSeedMenu)
    case Backup(MBackup)
    case PasswordConfirm(MPasswordConfirm)
    case SignatureReady(MSignatureReady)
    case EnterPassword(MEnterPassword)
    case LogRight(MLogRight)
    case NetworkDetailsMenu
    case ManageMetadata(MManageMetadata)
    case SufficientCryptoReady(MSufficientCryptoReady)
    case KeyDetailsAction
    case TypesInfo(MTypesInfo)
    case NewSeedBackup(MNewSeedBackup)
    case LogComment
    case SelectSeed(MSeeds)
}

/**
 * Alerts for showing
 *
 * This might be organized differently i iOS and android
 */
enum SignerAlert: Decodable {
    case Empty
    case Error(MError)
    case Shield
    case Confirm(MConfirm)
}

/**
 * All possible actions-buttons sent to backend are here
 * Some should be only pressed by model, not by user (e.g. those that need seed phrase or transaction)
 */
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
    case RemoveNetwork
    case RemoveMetadata
    case RemoveTypes
    case SignNetworkSpecs
    case SignMetadata
    case SignTypes
    case TextEntry
    case PushWord
    case ManageNetworks
    case ViewGeneralVerifier
    case ManageMetadata
    case RemoveKey
    case RemoveSeed
    case ClearLog
    case CreateLogComment
    case ShowLogDetails
    case Swipe
    case LongTap
    case SelectAll
    case Increment
    case ShowDocuments
    case ExportMultiSelect
}

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    func pushButton(action: Action, details: String = "", seedPhrase: String = "") {
        //Poor man's mutex; just because it's really managed by UI abstraction
        if actionAvailable {
            /** No returns below or things will stall! **/
            actionAvailable = false
            let res = backendAction(action: action, details: details, seedPhrase: seedPhrase)
            //print(String(cString: res!))
            if let actionResultJSON = res.data(using: .utf8) {
                //print(actionResultJSON)
                if let newActionResult = try? JSONDecoder().decode(ActionResult.self, from: actionResultJSON)
                {
                    //print(newActionResult)
                    actionResult = newActionResult
                } else {
                    print("pushing button failed on decoding!")
                    parsingAlert = true
                }
            }
            //Boink! debounce is here
            Timer.scheduledTimer(withTimeInterval: debounceTime, repeats: false, block: {_ in self.actionAvailable = true})
            /** Return is allowed again **/
        }
    }
}
