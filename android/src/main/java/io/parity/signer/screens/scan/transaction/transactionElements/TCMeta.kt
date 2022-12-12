package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.MMetadataRecord

@Composable
fun TCMeta(meta: TransactionMetadataModel) {
	Column {
		Text(
			stringResource(R.string.transaction_metadata_header),
			style = SignerTypeface.BodyL,
			color = MaterialTheme.colors.textSecondary,
			modifier = Modifier
				.padding(horizontal = 16.dp) //ios Spacing.medium
				.padding(bottom = 4.dp) //ios Spacing.extraExtraSmall
		)
		Column(
			verticalArrangement = Arrangement.spacedBy(8.dp),
			modifier = Modifier
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				)
				.padding(16.dp)
		) {
			Row() {
				Text(
					text = stringResource(R.string.transaction_metadata_label),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textSecondary,
				)
				Spacer(modifier = Modifier.weight(1f))
				Text(
					meta.specname,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary,
				)
				Spacer(modifier = Modifier.padding(end = 8.dp))
				Text(
					meta.specsVersion,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary,
				)
			}
//			TCNameValueElement( //todo dmitry as in other screens
//				name = stringResource(R.string.transaction_metadata_label),
//				value = "${meta.specname} ${meta.specsVersion}"
//			)
			SignerDivider()
			Text(
				meta.metaHash,
				style = SignerTypeface.BodyL,
				color = MaterialTheme.colors.primary,
			)
		}
	}
}


/**
 * Local copy of shared [MMetadataRecord] class
 */
data class TransactionMetadataModel(
	val specname: String,
	val specsVersion: String,
	val metaHash: String,
) {
	companion object {
		fun createStub(): TransactionMetadataModel =
			TransactionMetadataModel(
				specname = "Westend",
				specsVersion = "9230",
				metaHash = "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
			)
	}
}

fun MMetadataRecord.toTransactionMetadataModel(): TransactionMetadataModel =
	TransactionMetadataModel(
		specname = specname,
		specsVersion = specsVersion,
		metaHash = metaHash,
	)


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTCMeta() {
	val model = TransactionMetadataModel.createStub()
	SignerNewTheme {
		TCMeta(model)
	}
}


