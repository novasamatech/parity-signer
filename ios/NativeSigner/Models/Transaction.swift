//
//  Transaction.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation
import SwiftUI //To generate UIImage from raw png

/**
 * Transaction operations
 */
extension SignerDataModel {
    /**
     * Clears all transaction data; should be called on all resets, cancels, etc.
     */
    func resetTransaction() {
        self.transactionState = .none
        self.cards = []
        self.payloadStr = ""
        self.transactionError = ""
        self.action = nil
        self.qr = nil
        self.result = nil //TODO: remove this?
        self.author = nil
        self.comment = ""
    }
    
    /**
     * Parse decoded payload from QR parser (saved in the model)
     */
    func parse() {
        var err = ExternError()
        let err_ptr = UnsafeMutablePointer(&err)
        let res = parse_transaction(err_ptr, payloadStr, dbName)
        if err_ptr.pointee.code == 0 {
            if let cardsJSON = String(cString: res!).data(using: .utf8) {
                guard let transactionPreview = try? JSONDecoder().decode(TransactionCardSet.self, from: cardsJSON)
                else {
                    print("JSON decoder failed on transaction cards")
                    print(String(cString: res!))
                    signer_destroy_string(res!)
                    self.transactionState = .none
                    return
                }
                signer_destroy_string(res!)
                self.cards = []
                self.cards.append(contentsOf: (transactionPreview.warning ?? []))
                self.cards.append(contentsOf: (transactionPreview.types_info ?? []))
                self.cards.append(contentsOf: (transactionPreview.author ?? []))
                self.cards.append(contentsOf: (transactionPreview.error ?? []))
                self.cards.append(contentsOf: (transactionPreview.extrinsics ?? []))
                self.cards.append(contentsOf: (transactionPreview.method ?? []))
                self.cards = self.cards.sorted(by: {$0.index < $1.index})
                //print(self.cards)
                self.action = transactionPreview.action
                if transactionPreview.author != nil {
                    let authorCard = transactionPreview.author![0].card
                    switch authorCard {
                    case .author(let authorValue):
                        self.author = authorValue
                    default:
                        print("author not found; should not be actionable")
                    }
                }
                //print(self.author ?? "no author")
                //print(self.cards)
                self.transactionState = .preview
            } else {
                signer_destroy_string(res!)
                print("cards JSON corrupted!")
                self.transactionState = .none
            }
        } else {
            self.transactionError = String(cString: err_ptr.pointee.message)
            print(self.transactionError)
            signer_destroy_string(err_ptr.pointee.message)
            self.transactionState = .none
        }
    }
    
    /**
     * Handle action of whatever was parsed from payload and shown to user
     * If it is not a transaction, keep seedPhrase and password blank
     * otherwise fill seedPhrase from keyring and query user for password if it is set
     */
    func signTransaction(seedPhrase: String, password: String) {
        var err = ExternError()
        let err_ptr = UnsafeMutablePointer(&err)
        //TODO!!!
        let checksum = self.action?.payload.checksum
        let res = handle_sign(err_ptr, checksum, seedPhrase, password, Data(self.comment.utf8).base64EncodedString(), self.dbName)
        if err_ptr.pointee.code == 0 {
            self.result = String(cString: res!)
            signer_destroy_string(res!)
            if let imageData = Data(fromHexEncodedString: self.result ?? "") {
                self.qr = UIImage(data: imageData)
            } else {
                self.transactionError = "QR code generation error"
            }
        } else {
            self.transactionError = String(cString: err_ptr.pointee.message)
            print(self.transactionError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
