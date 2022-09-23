package io.parity.signer.components2.base

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.fill18

@Composable
fun CloseIcon(onClicked: () -> Unit) {
	Box(
		modifier = Modifier
			.size(32.dp)
			.background(
				MaterialTheme.colors.fill18,
				RoundedCornerShape(50)
			),
		contentAlignment = Alignment.Center,
	) {
		Image(
			imageVector = Icons.Filled.Close,
			contentDescription = stringResource(R.string.description_close_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.size(20.dp)
				.clickable { onClicked() }
		)
	}
}
