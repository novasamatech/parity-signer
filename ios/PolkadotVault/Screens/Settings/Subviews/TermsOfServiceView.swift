//
//  TermsOfServiceView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/01/2023.
//

import SwiftUI

struct TermsOfServiceView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.TermsOfService.Label.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: viewModel.onBackTap
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                )
            )
            ScrollView {
                Text(TextResources.termsAndConditions.text)
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .padding(.horizontal, Spacing.large)
                    .padding(.vertical, Spacing.medium)
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

extension TermsOfServiceView {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool

        init(isPresented: Binding<Bool>) {
            _isPresented = isPresented
        }

        func onBackTap() {
            isPresented = false
        }
    }
}

#if DEBUG
    struct TermsOfServiceView_Previews: PreviewProvider {
        static var previews: some View {
            TermsOfServiceView(viewModel: .init(isPresented: .constant(true)))
        }
    }
#endif
