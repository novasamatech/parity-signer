package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.QuestionMark
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.uniffi.MscFieldName
import io.parity.signer.uniffi.MscFieldNumber


@Composable
fun TCValueWithMarkdownTrio(
	value: TCWithTrioMarkdownModel,
) {
	var showDoc by remember {
		mutableStateOf(false)
	}

	val hasDetails: Boolean =
		(value.docsFieldName + value.pathType + value.docsType).isNotEmpty()

	Column(
		modifier = Modifier
			.animateContentSize()
			.run {
				if (hasDetails) {
					clickable { showDoc = !showDoc }
				} else {
					this
				}
			},
	) {
		Row(Modifier.fillMaxWidth(1f)) {
			Text(
				value.name,
				style = SignerTypeface.BodyL,
				color = MaterialTheme.colors.primary
			)
			if (hasDetails) {
				Image(
					imageVector = Icons.Default.QuestionMark,
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
					modifier = Modifier
						.padding(horizontal = 8.dp)
						.size(16.dp)
						.align(Alignment.CenterVertically)
				)
			}
		}
		if (showDoc) {
			Column(modifier = Modifier.padding(16.dp)) {
				Text(
					text = stringResource(
						id = R.string.transaction_field_path,
						value.pathType
					),
					style = SignerTypeface.BodyL,
					color = if (value.isNumber) MaterialTheme.colors.pink300 else MaterialTheme.colors.primary,
				)
				//todo markdowns below
				Text(
					text = value.docsFieldName,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary,
				)
				Text(
					text = value.docsType,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary,
				)
			}
		}
	}
}


/**
 * Local copy of shared [MscFieldName] and [MscFieldNumber] class
 *
 * @param [isNumber] if model is TCFieldNumber, false if TCFieldName - just different color
 */
data class TCWithTrioMarkdownModel(
	val name: String,
	val docsFieldName: String,
	val pathType: String,
	val docsType: String,
	val isNumber: Boolean,
) {
	companion object {
		fun createStub(): TCWithTrioMarkdownModel =
			TCWithTrioMarkdownModel(
				name = "method name",
				docsFieldName = "docs Field Numbar",
				pathType = "pathTYpe",
	//todo dmitry			//ios/NativeSigner/Models/Utils.swift:38
				docsType = PreviewData.exampleMarkdownDocs,
				isNumber = false
			)
	}
}

fun MscFieldName.toTCFieldNameModel() = TCWithTrioMarkdownModel(
	name = name,
	docsFieldName = docsFieldName,
	pathType = pathType,
	docsType = docsType,
	isNumber = false,
)

fun MscFieldNumber.toTCFieldNameModel() = TCWithTrioMarkdownModel(
	name = number,
	docsFieldName = docsFieldNumber,
	pathType = pathType,
	docsType = docsType,
	isNumber = true,
)


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTCFieldName() {
	SignerNewTheme {
		Column {
			TCValueWithMarkdownTrio(
				TCWithTrioMarkdownModel.createStub().copy(isNumber = false)
			)
			SignerDivider()
			TCValueWithMarkdownTrio(
				TCWithTrioMarkdownModel.createStub().copy(isNumber = true)
			)
		}
	}
}
