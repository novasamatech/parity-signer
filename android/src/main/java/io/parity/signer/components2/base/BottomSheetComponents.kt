package io.parity.signer.components2.base

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.*

@Composable
fun PrimaryButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	onClicked: () -> Unit,
) {
	Column(
		modifier = modifier
			.clickable(onClick = onClicked)
			.background(
				MaterialTheme.colors.pink500, RoundedCornerShape(
					dimensionResource(id = R.dimen.buttonCornerRadius)
				)
			)
			.fillMaxWidth()
			.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding)),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Text(
			text = label,
			color = Color.White,
			style = TypefaceNew.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun SecondaryButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	onClicked: () -> Unit,
) {
	Column(
		modifier = modifier
			.clickable(onClick = onClicked)
			.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
			.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Text(
			text = label,
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun RowButtonsBottomSheet(
	labelCancel: String,
	labelCta: String,
	isDangerCta: Boolean = false,
	onClickedCancel: () -> Unit,
	onClickedCta: () -> Unit,
) {
	Row {
		Column(
			modifier = Modifier
				.clickable(onClick = onClickedCancel)
				.background(
					MaterialTheme.colors.fill18, RoundedCornerShape(
						dimensionResource(id = R.dimen.buttonCornerRadius)
					)
				)
				.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
				.weight(1f),
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			Text(
				text = labelCancel,
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.TitleS,
				maxLines = 1,
			)
		}

		Spacer(modifier = Modifier.padding(horizontal = 8.dp))
		Column(
			modifier = Modifier
				.clickable(onClick = onClickedCta)
				.background(
					color = if (isDangerCta) MaterialTheme.colors.red400 else MaterialTheme.colors.pink500,
					shape = RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius))
				)
				.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
				.weight(1f),
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			Text(
				text = labelCta,
				color = Color.White,
				style = TypefaceNew.TitleS,
				maxLines = 1,
			)
		}
	}
}


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
private fun PreviewCtaButtons() {
	SignerNewTheme {
		Column(
			modifier = Modifier.size(300.dp),
		) {
			PrimaryButtonBottomSheet("button") {}
			RowButtonsBottomSheet(
				labelCancel = "Cancel",
				labelCta = "Apply",
				onClickedCancel = { },
				onClickedCta = {},
			)
		}
	}
}
