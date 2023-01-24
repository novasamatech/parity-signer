//
//  PrivacyPolicyView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/01/2023.
//

import SwiftUI

struct PrivacyPolicyView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.PrivacyPolicy.Label.title.string,
                    leftButton: .arrow,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    leftBarMenuAction: viewModel.onBackTap
                )
            )
            ScrollView {
                Text(ShownDocument.privacyPolicy.text)
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .padding(.horizontal, Spacing.large)
                    .padding(.vertical, Spacing.medium)
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

extension PrivacyPolicyView {
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
    struct PrivacyPolicyView_Previews: PreviewProvider {
        static var previews: some View {
            PrivacyPolicyView(viewModel: .init(isPresented: .constant(true)))
        }
    }
#endif
