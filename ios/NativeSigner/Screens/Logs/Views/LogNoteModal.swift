//
//  LogNoteModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/01/2023.
//

import SwiftUI

struct LogNoteModal: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
    @StateObject var viewModel: ViewModel
    @FocusState var focused: Bool
    @State var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .top
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Asset.xmarkButtonMedium.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .frame(
                                width: Heights.navigationButton,
                                height: Heights.navigationButton,
                                alignment: .center
                            )
                    }
                    .padding(.leading, Spacing.extraExtraSmall)
                    Spacer()
                    CapsuleButton(
                        action: viewModel.onDoneTap,
                        title: Localizable.LogsList.Modal.AddNote.Action.done.string,
                        isDisabled: $viewModel.isActionDisabled
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.LogsList.Modal.AddNote.Label.title.text
                        .font(PrimaryFont.titleM.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.medium)
                    Localizable.LogsList.Modal.AddNote.Label.header.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .padding(.bottom, Spacing.small)
                    TextEditor(text: $viewModel.note)
                        .frame(minHeight: Heights.minTextEditorHeight, maxHeight: Heights.maxTextEditorHeight)
                        .hiddenTextEditorBackground()
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .autocorrectionDisabled()
                        .autocapitalization(.none)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.return)
                        .padding(.horizontal, Spacing.small)
                        .background(Asset.fill6.swiftUIColor)
                        .cornerRadius(CornerRadius.small)
                        .submitLabel(.done)
                        .focused($focused)
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            }
            .background(Asset.backgroundTertiary.swiftUIColor)
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            focused = true
        }
    }
}

extension LogNoteModal {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        @Binding var isPresented: Bool
        @Published var note: String = ""
        @Published var isActionDisabled: Bool = true
        private var cancelBag = CancelBag()

        init(
            isPresented: Binding<Bool>
        ) {
            _isPresented = isPresented
            UITextView.appearance().backgroundColor = .green
            subscribeToUpdates()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
            navigation.performFake(navigation: .init(action: .createLogComment))
        }

        func onCancelTap() {
            isPresented.toggle()
        }

        func onDoneTap() {
            navigation.perform(navigation: .init(action: .goForward, details: note))
            navigation.perform(navigation: .init(action: .navbarLog))
            isPresented.toggle()
        }

        private func subscribeToUpdates() {
            $note.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct LogNoteModal_Previews: PreviewProvider {
        static var previews: some View {
            LogNoteModal(
                viewModel: .init(
                    isPresented: .constant(true)
                )
            )
            .environmentObject(NavigationCoordinator())
            .preferredColorScheme(.light)
        }
    }
#endif
