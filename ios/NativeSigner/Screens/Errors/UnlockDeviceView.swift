//
//  UnlockDeviceView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/01/2023.
//

import SwiftUI

struct UnlockDeviceView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var data: SharedDataModel

    var body: some View {
        VStack(spacing: 0) {
            Spacer()
            Asset.lockOpen.swiftUIImage
                .padding(.bottom, Spacing.extraExtraLarge)
            Localizable.Error.LockedDevice.Label.title.text
                .font(PrimaryFont.titleL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .padding(.horizontal, Spacing.x3Large)
                .padding(.bottom, Spacing.medium)
            Localizable.Error.LockedDevice.Label.subtitle.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
        .onAppear { viewModel.use(data: data) }
        .multilineTextAlignment(.center)
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

extension UnlockDeviceView {
    final class ViewModel: ObservableObject {
        private weak var data: SharedDataModel!
        private let seedsMediator: SeedsMediating

        init(seedsMediator: SeedsMediating = ServiceLocator.seedsMediator) {
            self.seedsMediator = seedsMediator
        }

        func use(data: SharedDataModel) {
            self.data = data
        }

        func onUnlockTap() {
            seedsMediator.refreshSeeds()
            data.totalRefresh()
        }
    }
}

struct UnlockDeviceView_Previews: PreviewProvider {
    static var previews: some View {
        UnlockDeviceView(viewModel: .init())
    }
}
