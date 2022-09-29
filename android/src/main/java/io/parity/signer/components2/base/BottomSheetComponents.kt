package io.parity.signer.components2.base

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.pink500

@Composable
fun CtaButtonBottomSheet(
	ctaString: String,
	modifier: Modifier = Modifier,
	onClicked: () -> Unit,
) {
	Column(
		modifier = modifier
			.background(MaterialTheme.colors.pink500, RoundedCornerShape(40.dp))
			.fillMaxWidth()
			.padding(vertical = 16.dp, horizontal = 24.dp)
			.clickable(onClick = onClicked),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Text(
			text = ctaString,
			color = MaterialTheme.colors.primary,
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
			.padding(vertical = 16.dp, horizontal = 24.dp)
			.fillMaxWidth()
			.clickable(onClick = onClicked),
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
