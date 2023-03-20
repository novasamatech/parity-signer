package io.parity.signer.screens.scan.transaction.components

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyCardSignature
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueElement
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textTertiary

@Composable
fun TransactionSummaryView(
	model: SigningTransactionModel,
	onTransactionClicked: (mTransactionInex: Int) -> Unit,
) {
	val plateShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
	Column(
		modifier = Modifier
			.padding(vertical = 8.dp, horizontal = 16.dp)
			.background(
				MaterialTheme.colors.fill6,
				plateShape
			)
			.padding(16.dp)
	) {
		Text(
			text = stringResource(R.string.transaction_summary_field_transaction_details),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier.padding(bottom = 8.dp)
		)
		model.summaryModels.forEach { summary ->
			Row(
				modifier = Modifier.clickable(onClick = {
					onTransactionClicked(summary.mTransactionIndex)
				}),
				verticalAlignment = Alignment.CenterVertically,
			) {
				//elements
				Column() {
					TCNameValueElement(
						name = stringResource(R.string.transaction_summary_field_pallet),
						value = summary.pallet
					)
					TCNameValueElement(
						name = stringResource(R.string.transaction_summary_field_method),
						value = summary.method
					)
					TCNameValueElement(
						name = stringResource(R.string.transaction_summary_field_destination),
						value = summary.destination
					)
					TCNameValueElement(
						name = stringResource(R.string.transaction_summary_field_value),
						value = summary.value
					)
				}
				Spacer(modifier = Modifier.weight(1f))
				//chervon
				Image(
					imageVector = Icons.Filled.ChevronRight,
					contentDescription = stringResource(R.string.transaction_summary_field_chrvron_description),
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
					modifier = Modifier
						.size(28.dp)
						.padding(end = 8.dp)
				)
			}
			SignerDivider(
				modifier = Modifier.padding(vertical = 8.dp),
				sidePadding = 0.dp
			)
		}
		model.keyModel?.let { keyModel ->
			Column() {
				Text(
					text = stringResource(R.string.transaction_summary_field_sign_with),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.CaptionM,
					modifier = Modifier.padding(bottom = 8.dp)
				)
				KeyCardSignature(model = keyModel)
			}
		}
	}
}


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
		TransactionSummaryView(SigningTransactionModel.createStub()) {}
	}
}
