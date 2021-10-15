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
enum SignerScreen: Equatable {
    case scan
    case keys
    case settings
    case history
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

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    /**
     * Event for back action
     * Could be more complicated but should it?
     */
    func goBack() {
        switch self.signerScreen {
        case .history:
            self.selectedRecord = nil
        case .scan:
            self.transactionState = .none
        case .keys:
            switch self.keyManagerModal {
            case .none:
                self.keyManagerModal = .seedSelector
            case .newSeed:
                self.keyManagerModal = .seedSelector
            default:
                self.keyManagerModal = .none
            }
        case .settings:
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
        case .scan:
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
        case .keys:
            return ""
        case .settings:
            return ""
        case .history:
            return ""
        }
    }
}
