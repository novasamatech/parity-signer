//
//  KeyboardOffsetAdapter.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/11/2022.
//

import Foundation
import UIKit

final class KeyboardOffsetAdapter: ObservableObject {
    @Published var keyboardHeight: CGFloat = 0

    init() {
        listenForKeyboardNotifications()
    }

    private func listenForKeyboardNotifications() {
        NotificationCenter.default.addObserver(
            forName: UIResponder.keyboardDidShowNotification,
            object: nil,
            queue: .main
        ) { notification in
            guard let userInfo = notification.userInfo,
                  let keyboardRect =
                  userInfo[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect
            else { return }

            if self.keyboardHeight == 0 {
                self.keyboardHeight = keyboardRect.height
            }
        }

        NotificationCenter.default.addObserver(
            forName: UIResponder.keyboardDidHideNotification,
            object: nil,
            queue: .main
        ) { _ in
            self.keyboardHeight = 0
        }
    }
}
