package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.HelpOutline
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.MarkdownText
import io.parity.signer.components.base.RichTextString
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.base.toRichTextStr
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
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
		value.docsFieldName.string.isNotEmpty() || value.pathType.isNotEmpty()
			|| value.docsType.string.isNotEmpty()

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
				Spacer(modifier = Modifier.weight(1f))
				Image(
					imageVector = Icons.Filled.HelpOutline,
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
			val innerShape =
				RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
			Column(
				modifier = Modifier
					.border(
						BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
						innerShape
					)
					.background(MaterialTheme.colors.fill6, innerShape)
					.padding(16.dp),
			) {
				Text(
					text = stringResource(
						id = R.string.transaction_field_path,
						value.pathType
					),
					style = SignerTypeface.BodyL,
					color = if (value.isNumber) MaterialTheme.colors.pink300 else MaterialTheme.colors.primary,
				)
				MarkdownText(content = value.docsFieldName)
				MarkdownText(content = value.docsType)
//				Text(
//					text = value.docsFieldName,
//					style = SignerTypeface.BodyL,
//					color = MaterialTheme.colors.primary,
//				)
//				Text(
//					text = value.docsType,
//					style = SignerTypeface.BodyL,
//					color = MaterialTheme.colors.primary,
//				)
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
	val docsFieldName: RichTextString,
	val pathType: String,
	val docsType: RichTextString,
	val isNumber: Boolean,
) {
	companion object {
		fun createStub(): TCWithTrioMarkdownModel =
			TCWithTrioMarkdownModel(
				name = "method name",
				docsFieldName = RichTextString("docs Field Numbar"),
				pathType = "pathTYpe",
				docsType = PreviewData.exampleMarkdownDocs,
				isNumber = false
			)
	}
}

fun MscFieldName.toTCFieldNameModel() = TCWithTrioMarkdownModel(
	name = name,
	docsFieldName = docsFieldName.toRichTextStr(),
	pathType = pathType,
	docsType = docsType.toRichTextStr(),
	isNumber = false,
)

fun MscFieldNumber.toTCFieldNameModel() = TCWithTrioMarkdownModel(
	name = number,
	docsFieldName = docsFieldNumber.toRichTextStr(),
	pathType = pathType,
	docsType = docsType.toRichTextStr(),
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
