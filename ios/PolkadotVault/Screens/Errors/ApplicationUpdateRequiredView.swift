//
//  ApplicationUpdateRequiredView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 02/10/2023.
//

import SwiftUI

struct ApplicationUpdateRequiredView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                Spacer()
                Image(.updateApp)
                    .padding(.bottom, Spacing.extraLarge)
                Localizable.Error.ApplicationUpdateRequired.Label.title.text
                    .font(
                        UIScreen.main.bounds.width == DeviceConstants.compactDeviceWidth ? PrimaryFont.titleM
                            .font : PrimaryFont.titleL.font
                    )
                    .foregroundColor(.textAndIconsPrimary)
                    .padding(
                        .horizontal,
                        UIScreen.main.bounds.width == DeviceConstants.compactDeviceWidth ? Spacing.large : Spacing
                            .extraExtraLarge
                    )
                    .padding(.bottom, Spacing.medium)
                Localizable.Error.ApplicationUpdateRequired.Label.subtitle.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(.textAndIconsTertiary)
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.large)
                Text(Localizable.applicationUpdateRequiredInfo())
                    .foregroundColor(.textAndIconsPrimary)
                    .multilineTextAlignment(.leading)
                    .font(PrimaryFont.bodyL.font)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(Spacing.medium)
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.extraLarge)
                ActionButton(
                    action: viewModel.onBackupTap,
                    text: Localizable.Error.ApplicationUpdateRequired.Action.backup.key,
                    style: .primary()
                )
                .padding(.horizontal, Spacing.large)
                Spacer()
                NavigationLink(
                    destination: BackupSelectKeyView(viewModel: .init())
                        .navigationBarHidden(true),
                    isActive: $viewModel.isBackupPresented
                ) { EmptyView() }
            }
            .multilineTextAlignment(.center)
            .background(.backgroundPrimary)
            .navigationBarHidden(true)
            .navigationViewStyle(.stack)
        }
    }
}

extension ApplicationUpdateRequiredView {
    final class ViewModel: ObservableObject {
        @Published var isBackupPresented: Bool = false

        init() {}

        func onBackupTap() {
            isBackupPresented = true
        }
    }
}

#if DEBUG
    struct ApplicationUpdateRequiredView_Previews: PreviewProvider {
        static var previews: some View {
            ApplicationUpdateRequiredView(viewModel: .init())
        }
    }
#endif
