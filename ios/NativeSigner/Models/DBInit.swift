//
//  DBInit.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.7.2021.
//

import Foundation

struct OnBoardingStruct {
    var done: Bool
    
    init() {
        do {
            let rootDir = try FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false)
            let contents = try FileManager.default.contentsOfDirectory(at: rootDir, includingPropertiesForKeys: [], options: FileManager.DirectoryEnumerationOptions.init())
            self.done = FileManager.default.fileExists(atPath: NSHomeDirectory() + "/Documents/Database")
        } catch {
            print("fileManager failed")
            self.done = false
        }
    }
    
    mutating func onboard() {
        do {
            if let source = Bundle.main.url(forResource: "Database", withExtension: "") {
                print(source)
                var destination = try FileManager.default.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: false)
                print(destination)
                print(destination.appendPathComponent("Database"))
                print(destination)
                try FileManager.default.copyItem(at: source, to: destination)
                self.done = true
            }
        } catch {
            print("DB init failed")
        }
    }
}
