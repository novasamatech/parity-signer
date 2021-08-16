//
//  Transaction.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation
import SwiftUI

enum TransactionState {
    case scanning
    case parsing
    case preview
    case show
}

class Transaction: ObservableObject {
    @Published var state: TransactionState = .scanning
    @Published var cards: [TransactionCard] = []
    @Published var payloadStr: String = ""
    @Published var dbName: String
    @Published var transactionError: String = ""
    @Published var action: Action?
    @Published var qr: UIImage?
    @Published var result: String? //TODO: remove this?
    @Published var author: Author?
    
    //var transactionPreview: TransactionCardSet?
    
    init() {
        self.dbName = NSHomeDirectory() + "/Documents/Database"
    }
    
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
                    self.state = .scanning
                    return
                }
                signer_destroy_string(res!)
                self.cards.append(contentsOf: (transactionPreview.warning ?? []))
                self.cards.append(contentsOf: (transactionPreview.types_info ?? []))
                self.cards.append(contentsOf: (transactionPreview.author ?? []))
                self.cards.append(contentsOf: (transactionPreview.error ?? []))
                self.cards.append(contentsOf: (transactionPreview.extrinsics ?? []))
                self.cards.append(contentsOf: (transactionPreview.method ?? []))
                self.cards = self.cards.sorted(by: {$0.index < $1.index})
                print(self.cards)
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
                print(self.author ?? "no author")
                self.state = .preview
            } else {
                signer_destroy_string(res!)
                print("cards JSON corrupted!")
                self.state = .scanning
            }
            
        } else {
            self.transactionError = String(cString: err_ptr.pointee.message)
            print(self.transactionError)
            signer_destroy_string(err_ptr.pointee.message)
            self.state = .scanning
        }
    }
    
    func signTransaction(seedPhrase: String, password: String) {
        var err = ExternError()
        let err_ptr = UnsafeMutablePointer(&err)
        guard let dataAction = try? JSONEncoder().encode(self.action!.payload) else {
            return
        }
        let stringAction = String(data: dataAction, encoding: .utf8)
        let res = handle_action(err_ptr, stringAction, seedPhrase, password, self.dbName)
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
