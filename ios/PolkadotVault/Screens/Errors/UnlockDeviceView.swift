//
//  UnlockDeviceView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/01/2023.
//

import SwiftUI

struct UnlockDeviceView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(spacing: 0) {
            Spacer()
            Image(.lockOpen)
                .padding(.bottom, Spacing.extraExtraLarge)
            Localizable.Error.LockedDevice.Label.title.text
                .font(PrimaryFont.titleL.font)
                .foregroundColor(.textAndIconsPrimary)
                .padding(.horizontal, Spacing.x3Large)
                .padding(.bottom, Spacing.medium)
            Localizable.Error.LockedDevice.Label.subtitle.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsTertiary)
                .padding(.horizontal, Spacing.extraExtraLarge)
                .padding(.bottom, Spacing.extraExtraLarge)
            PrimaryButton(
                action: viewModel.onUnlockTap,
                text: Localizable.Error.LockedDevice.Action.unlock.key,
                style: .primary()
            )
            .padding(.horizontal, Spacing.large)
            Spacer()
        }
        .multilineTextAlignment(.center)
        .background(.backgroundPrimary)
    }
}

extension UnlockDeviceView {
    final class ViewModel: ObservableObject {
        private let seedsMediator: SeedsMediating
        private let warningStateMediator: WarningStateMediator

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator
        ) {
            self.seedsMediator = seedsMediator
            self.warningStateMediator = warningStateMediator
        }

        func onUnlockTap() {
            seedsMediator.refreshSeeds()
            warningStateMediator.updateWarnings()
        }
    }
}

#if DEBUG
    struct UnlockDeviceView_Previews: PreviewProvider {
        static var previews: some View {
            UnlockDeviceView(viewModel: .init())
        }
    }
#endif
