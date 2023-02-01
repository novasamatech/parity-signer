//
//  DevicePincodeRequiredView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 27/01/2023.
//

import SwiftUI

struct DevicePincodeRequired: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack(spacing: 0) {
            Spacer()
            Asset.devicePincode.swiftUIImage
                .padding(.bottom, Spacing.extraExtraLarge)
            Localizable.Error.DevicePincodeRequired.Label.title.text
                .font(PrimaryFont.titleL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .padding(.horizontal, Spacing.x3Large)
                .padding(.bottom, Spacing.small)
            Localizable.Error.DevicePincodeRequired.Label.subtitle.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.medium)
            VStack(alignment: .leading, spacing: Spacing.small) {
                HStack(alignment: .top, spacing: 0) {
                    Text("1")
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .frame(width: Spacing.large, alignment: .leading)
                    Localizable.Error.DevicePincodeRequired.Label.step1.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .lineSpacing(Spacing.extraExtraSmall)
                }
                HStack(alignment: .top, spacing: 0) {
                    Text("2")
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .frame(width: Spacing.large, alignment: .leading)
                    Localizable.Error.DevicePincodeRequired.Label.step2.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .lineSpacing(Spacing.extraExtraSmall)
                }
            }
            .multilineTextAlignment(.leading)
            .font(PrimaryFont.bodyL.font)
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(Spacing.medium)
            .strokeContainerBackground()
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.extraExtraLarge)
            PrimaryButton(
                action: viewModel.onOpenTap,
                text: Localizable.Error.DevicePincodeRequired.Action.settings.key,
                style: .primary()
            )
            .padding(.horizontal, Spacing.large)
            Spacer()
        }
        .multilineTextAlignment(.center)
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

extension DevicePincodeRequired {
    final class ViewModel: ObservableObject {
        private let urlOpener: UIApplication

        init(urlOpener: UIApplication = UIApplication.shared) {
            self.urlOpener = urlOpener
        }

        func onOpenTap() {
            guard let settingsUrl = URL(string: UIApplication.openSettingsURLString),
                  urlOpener.canOpenURL(settingsUrl) else { return }
            urlOpener.open(settingsUrl)
        }
    }
}

struct DevicePincodeRequired_Previews: PreviewProvider {
    static var previews: some View {
        DevicePincodeRequired(viewModel: .init())
    }
}
