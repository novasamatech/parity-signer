package io.parity.signer.screens.scan.transaction.components

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueElement
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textTertiary

@Composable
fun TransactionSummaryView(model: SigningTransactionModel) {
	val plateShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
	Column(
		modifier = Modifier.background(
			MaterialTheme.colors.fill6,
			plateShape
		)
	) {
		Text(
			text = "Transaction Details", //todo scane
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier.padding(bottom = 8.dp)
		)
		model.summaryModels.forEach { summary ->
			Row() {
				//elements
				Column() {
					TCNameValueElement("Pallet", summary.pallet)
					TCNameValueElement("Method", summary.method)
					TCNameValueElement("Dest", summary.destination)
					TCNameValueElement("Value", summary.value)
				}
				Spacer(modifier = Modifier.weight(1f))
				//chervon
				Image(
					imageVector = Icons.Filled.ChevronRight,
					contentDescription = "Transaction details",
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
					modifier = Modifier
                        .size(28.dp)
                        .padding(end = 8.dp)
				)
			}
			SignerDivider() //todo scan made it in a center
		}
		//signature todo scan
	}
}


//struct TransactionSummaryView: View {
//    var renderable: TransactionPreviewRenderable
//    let onTransactionDetailsTap: () -> Void
//    @State var isShowingFullAddress: Bool = false
//
//var body: some View
//{
//	VStack(alignment:.leading, spacing: Spacing.extraSmall) {
//	VStack(alignment:.leading, spacing: 0) {
//	Localizable.TransactionSign.Label.details.text
//		.foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//		.font(PrimaryFont.captionM.font)
//		.padding(. bottom, Spacing.extraSmall)
//	HStack {
//		VStack(alignment:.leading, spacing: 0) {
//		ForEach(renderable.summary.asRenderable, id: \.id) {
//		row in
//			HStack(spacing: Spacing. extraSmall) {
//		Text(row.key)
//			.foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//		Text(row.value)
//			.foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
//	}
//		.font(PrimaryFont.bodyM.font)
//		.frame(minHeight: Heights. minTransactionSummaryItemHeight)
//	}
//	}
//		Spacer()
//		Asset.chevronRight.swiftUIImage
//			.foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//			.padding(Spacing.extraSmall)
//	}
//}
//	.contentShape(Rectangle())
//	.onTapGesture { onTransactionDetailsTap() }
//	signature()
//}
//	.padding(Spacing.medium)
//	.background(
//		RoundedRectangle(cornerRadius: CornerRadius. small
//	)
//	.fill(Asset.fill6.swiftUIColor)
//	)
//}
//
//    @ViewBuilder
//    func signature() -> some View {
//        if let signature = renderable.signature {
//            Divider()
//            VStack(alignment: .leading, spacing: 0) {
//                Localizable.TransactionSign.Label.sign.text
//                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//                    .font(PrimaryFont.captionM.font)
//                    .padding(.bottom, Spacing.extraSmall)
//                HStack {
//                    VStack(alignment: .leading, spacing: 2) {
//                        renderablePath(for: signature)
//                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//                            .font(PrimaryFont.captionM.font)
//                        Text(signature.name)
//                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
//                            .font(PrimaryFont.bodyM.font)
//                        HStack {
//                            Text(
//                                isShowingFullAddress ? signature.base58 : signature.base58
//                                    .truncateMiddle()
//                            )
//                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//                            .font(PrimaryFont.bodyM.font)
//
//                            if !isShowingFullAddress {
//                                Asset.chevronDown.swiftUIImage
//                                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
//                                    .padding(.leading, Spacing.extraExtraSmall)
//                            }
//                        }
//                        .contentShape(Rectangle())
//                        .onTapGesture {
//                            withAnimation {
//                                isShowingFullAddress = true
//                            }
//                        }
//                    }
//                    Spacer()
//                    Identicon(identicon: signature.identicon, rowHeight: Heights.identiconInCell)
//                }
//            }
//        } else {
//            EmptyView()
//        }
//    }
//
//    /// Manual string interpolation for `lock` `SFSymbol`
//    private func renderablePath(for signature: TransactionSignatureRenderable) -> Text {
//        signature.hasPassword ?
//            Text("\(signature.path)\(Image(.lock))") :
//            Text(signature.path)
//    }
//}
//
//struct TransactionSummaryView_Previews: PreviewProvider {
//    static var previews: some View {
//        TransactionSummaryView(
//            renderable: .init(
//                summary: PreviewData.transactionSummary,
//                signature: PreviewData.transactionSignature
//            ),
//            onTransactionDetailsTap: {}
//        )
//        .preferredColorScheme(.dark)
//    }
//}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTransactionSummaryView() {
	SignerNewTheme {
		TransactionSummaryView(SigningTransactionModel.createStub())
	}
}
