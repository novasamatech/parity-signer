//
//  SetUpNetworksStepTwoView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import SwiftUI

struct SetUpNetworksStepTwoView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        GeometryReader { geo in
            VStack(alignment: .leading, spacing: 0) {
                NavigationBarView(
                    viewModel: .init(
                        leftButtons: [.init(
                            type: .arrow,
                            action: viewModel.onBackButtonTap
                        )],
                        rightButtons: [.init(
                            type: .action(
                                Localizable.Onboarding.SetUpNetworks.Step2.Action.done.key
                            ),
                            action: viewModel.onDoneButtonTap
                        )]
                    )
                )
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        // Header
                        Localizable.Onboarding.SetUpNetworks.Step2.Label.step.text
                            .font(PrimaryFont.captionM.font)
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .multilineTextAlignment(.leading)
                            .padding(.horizontal, Spacing.large)
                            .padding(.top, Spacing.medium)
                            .padding(.bottom, Spacing.extraExtraSmall)
                        Localizable.Onboarding.SetUpNetworks.Step2.Label.title.text
                            .font(PrimaryFont.titleL.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .padding(.horizontal, Spacing.large)
                            .padding(.bottom, Spacing.large)
                        // Tutorial
                        VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                            sectionOne()
                            sectionTwo()
                            sectionThree()
                        }
                        Spacer()
                    }
                }
            }
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
            .background(Asset.backgroundSystem.swiftUIColor)
        }
    }

    @ViewBuilder
    func pointCircle(_ number: String) -> some View {
        ZStack(alignment: .center) {
            Circle()
                .foregroundColor(Asset.backgroundPrimary.swiftUIColor)
                .frame(width: Sizes.pointCircle)
            Text(number)
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
        }
    }

    @ViewBuilder
    func sectionOne() -> some View {
        HStack(alignment: .top, spacing: Spacing.small) {
            pointCircle("1")
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.Onboarding.SetUpNetworks.Step2.Label.Step1.one.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Text(Localizable.setUpNetworkStepTwoStepPartTwo())
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
                Text(Localizable.setUpNetworkStepTwoStepPartThree())
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
            }
            Spacer()
        }
        .padding(Spacing.medium)
        .containerBackground(state: .textContainer)
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func sectionTwo() -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            pointCircle("2")
            Localizable.Onboarding.SetUpNetworks.Step2.Label.step2.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            Spacer()
        }
        .padding(Spacing.medium)
        .containerBackground(state: .textContainer)
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func sectionThree() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            HStack(alignment: .top, spacing: Spacing.small) {
                pointCircle("3")
                Localizable.Onboarding.SetUpNetworks.Step2.Label.step3.text
                    .font(PrimaryFont.bodyL.font)
            }
            SecondaryButton(
                action: viewModel.onScanTap(),
                text: Localizable.Onboarding.SetUpNetworks.Step2.Label.Step3.action.key,
                style: .secondary()
            )
            Spacer()
        }
        .padding(Spacing.medium)
        .containerBackground(state: .textContainer)
        .padding(.horizontal, Spacing.medium)
    }
}

extension SetUpNetworksStepTwoView {
    final class ViewModel: ObservableObject {
        private let onNextTap: () -> Void
        private let onBackTap: () -> Void
        @Published var isShowingQRScanner: Bool = false

        init(
            onNextTap: @escaping () -> Void,
            onBackTap: @escaping () -> Void
        ) {
            self.onNextTap = onNextTap
            self.onBackTap = onBackTap
        }

        func onBackButtonTap() {
            onBackTap()
        }

        func onDoneButtonTap() {
            onNextTap()
        }

        func onScanTap() {}
    }
}

#if DEBUG
    struct SetUpNetworksStepTwoView_Previews: PreviewProvider {
        static var previews: some View {
            SetUpNetworksStepTwoView(
                viewModel: .init(onNextTap: {}, onBackTap: {})
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
