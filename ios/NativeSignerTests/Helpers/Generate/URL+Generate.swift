//
//  URL+Generate.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

import Foundation

extension URL {
    static func generate(filePath: String = "path") -> URL! {
        URL(fileURLWithPath: "/System/" + filePath)
    }
}
