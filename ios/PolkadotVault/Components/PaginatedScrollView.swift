//
//  PaginatedScrollView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import SwiftUI

struct PaginatedScrollView: View {
    private let items: [AnyView]
    private let itemPadding: CGFloat
    private let itemSpacing: CGFloat
    private let itemWidth: CGFloat
    private let itemsAmount: Int
    private let contentWidth: CGFloat

    private let leadingOffset: CGFloat
    private let scrollDampingFactor: CGFloat = 0.6

    @Binding var currentPageIndex: Int

    @State private var currentScrollOffset: CGFloat = 0
    @State private var gestureDragOffset: CGFloat = 0

    init(
        currentPageIndex: Binding<Int>,
        itemsAmount: Int,
        itemWidth: CGFloat,
        itemPadding: CGFloat,
        pageWidth: CGFloat,
        @ViewBuilder content: () -> some View
    ) {
        let views = content()
        items = [AnyView(views)]

        _currentPageIndex = currentPageIndex

        self.itemsAmount = itemsAmount
        itemSpacing = itemPadding
        self.itemWidth = itemWidth
        self.itemPadding = itemPadding
        contentWidth = (itemWidth + itemPadding) * CGFloat(itemsAmount)

        let itemRemain = (pageWidth - itemWidth - 2 * itemPadding) / 2
        leadingOffset = itemRemain + itemPadding
    }

    private func countOffset(for pageIndex: Int) -> CGFloat {
        let activePageOffset = CGFloat(pageIndex) * (itemWidth + itemPadding)
        return leadingOffset - activePageOffset
    }

    private func countPageIndex(for offset: CGFloat) -> Int {
        guard itemsAmount > 0 else { return 0 }
        var index = Int(round(countLogicalOffset(offset) / (itemWidth + itemPadding)))
        // allow to change just 1 page offset at the time
        index = min(max(index, currentPageIndex - 1), currentPageIndex + 1)
        // keep in items range
        index = min(max(index, 0), itemsAmount - 1)
        return index
    }

    private func countCurrentScrollOffset() -> CGFloat {
        countOffset(for: currentPageIndex) + gestureDragOffset
    }

    private func countLogicalOffset(_ trueOffset: CGFloat) -> CGFloat {
        (trueOffset - leadingOffset) * -1.0
    }

    var body: some View {
        GeometryReader { _ in
            HStack(alignment: .center, spacing: itemSpacing) {
                ForEach(items.indices, id: \.self) { itemIndex in
                    items[itemIndex].frame(width: itemWidth)
                }
            }
        }
        .onAppear {
            currentScrollOffset = countOffset(for: currentPageIndex)
        }
        .background(
            Color.black
                .opacity(0.000001)
        ) // hack - this allows gesture recognizing even when background is transparent
        .frame(width: contentWidth)
        .offset(x: currentScrollOffset, y: 0)
        .simultaneousGesture(
            DragGesture(minimumDistance: 1, coordinateSpace: .local)
                .onChanged { value in
                    gestureDragOffset = value.translation.width
                    currentScrollOffset = countCurrentScrollOffset()
                }
                .onEnded { value in
                    let cleanOffset = (value.predictedEndTranslation.width - gestureDragOffset)
                    let velocityDiff = cleanOffset * scrollDampingFactor

                    let newPageIndex = countPageIndex(for: currentScrollOffset + velocityDiff)

                    gestureDragOffset = 0
                    withAnimation(
                        .interpolatingSpring(
                            mass: 0.1,
                            stiffness: 20,
                            damping: 1.5,
                            initialVelocity: 0
                        )
                    ) {
                        currentPageIndex = newPageIndex
                        currentScrollOffset = countCurrentScrollOffset()
                    }
                }
        )
    }
}
