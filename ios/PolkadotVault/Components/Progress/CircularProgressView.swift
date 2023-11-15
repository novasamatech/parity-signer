//
//  CircularProgressView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 01/09/2022.
//

import SwiftUI

struct CircularCountdownViewModel {
    let size: CGFloat
    let lineWidth: CGFloat
    let backgroundColor: Color
    let foregroundColor: Color
    let foregroundFontColor: Color

    /// Circular progress view configuration to be used in `Export Private Key` flow
    static let privateKeyCountdown = CircularCountdownViewModel(
        size: 32,
        lineWidth: 2.5,
        backgroundColor: .clear,
        foregroundColor: .accentPink300,
        foregroundFontColor: .textAndIconsPrimary
    )
    /// Circular progress view configuration to be used in `Snackbar` component
    static let snackbarCountdown = CircularCountdownViewModel(
        size: 32,
        lineWidth: 2.5,
        backgroundColor: .clear,
        foregroundColor: .accentPink300,
        foregroundFontColor: .accentForegroundText
    )
}

struct CircularCountdownModel {
    // For how many seconds countdown should go
    let counter: CGFloat
    /// View model for circular countdown
    let viewModel: CircularCountdownViewModel
    /// Action that should get triggered on countdown completion
    let onCompletion: () -> Void

    init(
        counter: CGFloat,
        viewModel: CircularCountdownViewModel,
        onCompletion: @escaping () -> Void
    ) {
        self.counter = counter
        self.viewModel = viewModel
        self.onCompletion = onCompletion
    }
}

struct CircularProgressView: View {
    private enum Constants {
        static let animationPrecision: CGFloat = 0.1
        static let startingAngle = Angle(degrees: 270)
    }

    private let model: CircularCountdownModel
    @State private var counter: CGFloat
    @State private var timer = Timer
        .publish(every: Constants.animationPrecision, on: .main, in: .common)
        .autoconnect()
    @EnvironmentObject var applicationStatePublisher: ApplicationStatePublisher
    @State private var lastApplicationState: ApplicationState?
    @State private var moveToBackgroundDate: Date?
    private let cancelBag = CancelBag()

    init(_ model: CircularCountdownModel) {
        self.model = model
        _counter = State(initialValue: model.counter)
    }

    var body: some View {
        ZStack {
            Circle()
                .stroke(
                    model.viewModel.backgroundColor,
                    lineWidth: model.viewModel.lineWidth
                )
            Circle()
                .trim(from: 0, to: normalisedProgress())
                .stroke(
                    model.viewModel.foregroundColor,
                    style: StrokeStyle(
                        lineWidth: model.viewModel.lineWidth,
                        lineCap: .round,
                        lineJoin: .round
                    )
                )
                .rotationEffect(Constants.startingAngle)
                .animation(.linear, value: normalisedProgress())
            Text(String(Int(counter)))
                .foregroundColor(model.viewModel.foregroundFontColor)
                .font(PrimaryFont.labelS.font)
        }
        .frame(width: model.viewModel.size, height: model.viewModel.size, alignment: .center)
        .onReceive(timer) { _ in
            if counter > 0 {
                counter = max(0, counter - Constants.animationPrecision)
            } else {
                timer.upstream.connect().cancel()
                model.onCompletion()
            }
        }
        .onAppear {
            applicationStatePublisher.$applicationState.sink { updatedState in
                guard lastApplicationState != updatedState, counter > 0 else { return }
                lastApplicationState = updatedState
                switch updatedState {
                case .active:
                    guard let moveToBackgroundDate else { return }
                    let timePassed = Date().timeIntervalSince(moveToBackgroundDate)
                    counter = max(0, counter - timePassed)
                    if counter < 0 {
                        timer.upstream.connect().cancel()
                        model.onCompletion()
                    }
                case .inactive:
                    moveToBackgroundDate = .now
                }
            }.store(in: cancelBag)
        }
    }

    private func normalisedProgress() -> CGFloat {
        (CGFloat(model.counter) - counter) / CGFloat(model.counter)
    }
}

#if DEBUG
    struct CircularProgressView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                CircularProgressView(
                    CircularCountdownModel(
                        counter: 15,
                        viewModel: .privateKeyCountdown,
                        onCompletion: {}
                    )
                )
            }
            .padding()
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack {
                CircularProgressView(
                    CircularCountdownModel(
                        counter: 15,
                        viewModel: .snackbarCountdown,
                        onCompletion: {}
                    )
                )
            }
            .padding()
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
            .environmentObject(ApplicationStatePublisher())
        }
    }
#endif
