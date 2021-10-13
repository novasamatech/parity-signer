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
}

/**
 * Modals shown in settings screen
 */
enum SettingsModal: Equatable {
    case none
    case showNetworkManager
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
        self.transactionState = .none
        self.keyManagerModal = .none
        self.settingsModal = .none
        self.selectedRecord = nil
    }
    
    /**
     * Returns true if back navigation button should not be shown
     */
    func isNavBottom() -> Bool {
        return (self.transactionState == .none && self.keyManagerModal == .none && self.settingsModal == .none && self.selectedRecord == nil)
    }
}
