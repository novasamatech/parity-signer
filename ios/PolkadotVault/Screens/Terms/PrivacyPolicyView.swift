//
//  PrivacyPolicyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/01/2023.
//

import SwiftUI

struct PrivacyPolicyView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(Localizable.Settings.PrivacyPolicy.Label.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: { viewModel.onBackTap?() ?? presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: .backgroundPrimary
                )
            )
            ScrollView {
                Text(TextResources.privacyPolicy.text)
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(.textAndIconsPrimary)
                    .padding(.horizontal, Spacing.large)
                    .padding(.vertical, Spacing.medium)
            }
        }
        .background(.backgroundPrimary)
    }
}

extension PrivacyPolicyView {
    final class ViewModel: ObservableObject {
        let onBackTap: (() -> Void)?

        init(onBackTap: (() -> Void)? = nil) {
            self.onBackTap = onBackTap
        }
    }
}

#if DEBUG
    struct PrivacyPolicyView_Previews: PreviewProvider {
        static var previews: some View {
            PrivacyPolicyView(viewModel: .init())
        }
    }
#endif
