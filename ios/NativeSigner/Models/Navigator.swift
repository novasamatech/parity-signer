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
    var screen: SignerScreen?
}

/**
 * Struct to store main navstate of the screen
 */
enum SignerScreen: String, Decodable {
    case Scan
    case Keys
    case Settings
    case Log
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
enum KeyManagerModal: Equatable {
    case none
    case newSeed
    case newKey
    case showKey
    case seedBackup
    case keyDeleteConfirm
    case seedSelector
    case networkManager
    case networkDetails
}

/**
 * Modals shown in settings screen
 */
enum SettingsModal: Equatable {
    case none
    case showDocument(ShownDocument)
}

enum ButtonID {
    case NavbarLog
    case NavbarScan
    case NavbarKeys
    case NavbarSettings
}

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    func pushButton(buttonID: ButtonID) {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = act(err_ptr, String(describing: self.signerScreen), String(describing: buttonID), "")
        if (err_ptr.pointee.code == 0) {
            print(String(cString: res!))
            if let actionResultJSON = String(cString: res!).data(using: .utf8) {
                print(actionResultJSON)
                if let actionResult = try? JSONDecoder().decode(ActionResult.self, from: actionResultJSON)
                {
                    print(actionResult)
                    if (actionResult.screen != nil) {
                        signerScreen = actionResult.screen!
                    }
                } else {
                    print("bushing button failed on decoding!")
                }
            }
            signer_destroy_string(res!)
        } else {
            print("pushing button failed")
        }
    }
    
    /**
     * Event for back action
     * Could be more complicated but should it?
     */
    func goBack() {
        switch self.signerScreen {
        case .Log:
            self.selectedRecord = nil
        case .Scan:
            self.transactionState = .none
        case .Keys:
            switch self.keyManagerModal {
            case .seedSelector:
                self.keyManagerModal = .seedSelector
            case .none:
                self.keyManagerModal = .seedSelector
            case .newSeed:
                self.keyManagerModal = .seedSelector
            case .seedBackup:
                self.keyManagerModal = .seedSelector
            default:
                self.keyManagerModal = .none
            }
        case .Settings:
            self.settingsModal = .none
        }
    }
    
    /**
     * Returns true if back navigation button should not be shown
     */
    func isNavBottom() -> Bool {
        return (self.transactionState == .none && self.keyManagerModal == .seedSelector && self.settingsModal == .none && self.selectedRecord == nil)
    }
    
    /**
     * Logic behind screen name in top bar
     */
    func getScreenName() -> String {
        switch self.signerScreen {
        case .Scan:
            switch self.transactionState {
            case .none:
                return "Scan"
            case .parsing:
                return "Parsing"
            case .preview:
                return "Payload"
            case .password:
                return "Password"
            case .signed:
                return "Scan to publish"
            }
        case .Keys:
            switch self.keyManagerModal {
            case .seedSelector:
                return "Select Seed"
            case .newKey:
                return "New Derived Key"
            case .showKey:
                return (self.selectedAddress?.isRoot() ?? false) ? "Seed Key" : "Derived Key"
            case .seedBackup:
                return "Backup seed"
            default:
                return ""
            }
        case .Settings:
            return ""
        case .Log:
            if self.selectedRecord == nil {
                return ""
            } else {
                return "Event"
            }
        }
    }
}
