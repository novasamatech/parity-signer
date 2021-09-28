//
//  Documents.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import Foundation

enum ShownDocument {
    case toc
    case pp
    case about
}

extension SignerDataModel {
    func getTaC() -> String {
        if let path = Bundle.main.path(forResource: "terms-and-conditions", ofType: "txt") {
            do {
                let tac = try String(contentsOfFile: path, encoding: .utf8)
                //TODO: let TaCMD = AttributedString(markdown: TaC)
                return tac
            } catch {
                print("TaC file damaged")
                return "Terms and conditions text corrupted! Please report bug."
            }
        } else {
            print("TaC file not found!")
            return "Terms and conditions not found! Please report bug."
        }
    }
    
    func getPP() -> String {
        if let path = Bundle.main.path(forResource: "privacy-policy", ofType: "txt") {
            do {
                let pp = try String(contentsOfFile: path, encoding: .utf8)
                //TODO: let TaCMD = AttributedString(markdown: TaC)
                return pp
            } catch {
                print("PP file damaged")
                return "Privacy policy text corrupted! Please report bug."
            }
        } else {
            print("PP file not found!")
            return "Privacy policy not found! Please report bug."
        }
    }
}
