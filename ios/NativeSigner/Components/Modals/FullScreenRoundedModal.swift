//
//  FullScreenRoundedModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/09/2022.
//

import SwiftUI

struct FullScreenRoundedModal<Content: View>: View {
    @Binding private var animateBackground: Bool
    private let backgroundTapAction: () -> Void
    private let content: () -> Content
    private let ignoredEdges: Edge.Set

    init(
        backgroundTapAction: @escaping () -> Void = {},
        animateBackground: Binding<Bool> = Binding<Bool>.constant(false),
        ignoredEdges: Edge.Set = .all,
        @ViewBuilder content: @escaping () -> Content
    ) {
        self.backgroundTapAction = backgroundTapAction
        self.ignoredEdges = ignoredEdges
        _animateBackground = animateBackground
        self.content = content
    }

    var body: some View {
        ZStack(alignment: .bottom) {
            // Semi transparent background
            Spacer()
                .frame(idealHeight: .infinity)
                .background(animateBackground ? Color.black.opacity(0.5) : .clear)
                .onTapGesture(perform: backgroundTapAction)
                .onAppear {
                    withAnimation(
                        Animation.easeIn(duration: AnimationDuration.standard)
                            .delay(AnimationDuration.standard)
                    ) {
                        animateBackground.toggle()
                    }
                }
            // Modal content
            VStack(alignment: .leading, spacing: 0) {
                Spacer().frame(height: Spacing.topSafeAreaSpacing)
                    .background(animateBackground ? Color.black.opacity(0.5) : .clear)
                    .onTapGesture(perform: backgroundTapAction)
                VStack(alignment: .leading, spacing: Spacing.medium, content: content)
                    .padding([.bottom, .top], Spacing.medium)
                    .padding([.leading, .trailing], 0)
                    .background(Asset.backgroundTertiary.swiftUIColor)
                    .cornerRadius(radius: CornerRadius.medium, corners: [.topLeft, .topRight])
            }
        }
        .ignoresSafeArea(edges: ignoredEdges)
    }
}

//
// struct FullScreenRoundedModal_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack {
//            FullScreenRoundedModal {
//                Text("Test label")
//                    .padding()
//                Text("Test label")
//                    .padding()
//            }
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack {
//            FullScreenRoundedModal {
//                Text("Test label")
//                    .padding()
//                Text("Test label")
//                    .padding()
//            }
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
// }
