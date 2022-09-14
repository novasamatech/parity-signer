//
//  Snackbar.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/09/2022.
//

import SwiftUI

final class BottomSnackbarPresentation: ObservableObject {
    @Published var viewModel: SnackbarViewModel = .init(title: "")
    @Published var isSnackbarPresented: Bool = false
}

struct SnackbarViewModel {
    let title: String
    let style: Snackbar.Style

    init(
        title: String,
        style: Snackbar.Style = .info
    ) {
        self.title = title
        self.style = style
    }
}

struct Snackbar: View {
    enum Style {
        case info
        case warning

        var tintColor: Color {
            switch self {
            case .info:
                return Asset.fill12Solid.swiftUIColor
            case .warning:
                return Asset.accentRed400.swiftUIColor
            }
        }
    }

    private let viewModel: SnackbarViewModel

    init(
        viewModel: SnackbarViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        VStack {
            Spacer()
            HStack {
                Text(viewModel.title)
                    .font(Fontstyle.bodyL.base)
                    .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                    .padding(Spacing.large)
                Spacer()
            }
            .frame(height: Heights.snackbarHeight, alignment: .center)
            .background(viewModel.style.tintColor)
            .cornerRadius(CornerRadius.small)
        }
        .padding()
    }
}

extension View {
    /// Presents given `overlayView` over bottom edge with opacity transition. Dismiss view with bottom edge transition
    /// - Parameters:
    ///   - overlayView: view to be presented as overlay
    ///   - isPresented: action controller in form of `Bool`
    /// - Returns: view that modifier is applied to
    func bottomSnackbar(_ viewModel: SnackbarViewModel, isPresented: Binding<Bool>) -> some View {
        bottomEdgeOverlay(overlayView: Snackbar(viewModel: viewModel), isPresented: isPresented)
            .tapAndDelayDismiss(isPresented: isPresented)
    }
}

// struct SnackbarDemo: View {
//    @State private var showInfo = false
//    @State private var showWarning = false
//
//    var body: some View {
//        VStack {
//            Text("Present info snackbar")
//                .onTapGesture {
//                    showInfo = true
//                }
//            Spacer()
//        }.bottomSnackbar(
//            SnackbarViewModel(title: "Metadata has been updated", style: .info),
//            isPresented: $showInfo
//        )
//    }
// }
//
//
// struct Snackbar_Previews: PreviewProvider {
//    @State private var showOverlay = false
//
//    static var previews: some View {
//        SnackbarDemo()
//            .preferredColorScheme(.light)
//    }
// }
//
