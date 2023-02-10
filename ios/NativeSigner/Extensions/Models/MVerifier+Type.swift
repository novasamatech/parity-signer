//
//  MVerifier+Type.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 10/02/2023.
//

import Foundation

extension MVerifier {
    public enum VerifierType: String, RawRepresentable {
        private enum RustValues: String {
            case general
            case custom
            case none
        }

        case general
        case none
        case custom
        case unknown

        public init(rawValue: String) {
            switch rawValue {
            case RustValues.general.rawValue:
                self = .general
            case RustValues.none.rawValue:
                self = .none
            case RustValues.custom.rawValue:
                self = .custom
            default:
                self = .unknown
            }
        }

        public var rawValue: String {
            switch self {
            case .general:
                return RustValues.general.rawValue
            case .none:
                return RustValues.none.rawValue
            case .custom:
                return RustValues.custom.rawValue
            case .unknown:
                return "Unknown"
            }
        }
    }

    var type: VerifierType {
        VerifierType(rawValue: ttype)
    }
}
