package io.parity.signer.components.base

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.fill18

@Composable
fun CloseIcon(
	modifier: Modifier = Modifier,
	noBackground: Boolean = false,
	onCloseClicked: Callback
) {
	Box(
		modifier = modifier
			.size(32.dp)
			.clip(CircleShape)
			.clickable(onClick = onCloseClicked)
			.run {
				if (noBackground) {
					this
				} else {
					background(
						MaterialTheme.colors.fill18,
					)
				}
			},
		contentAlignment = Alignment.Center,
	) {
		Image(
			imageVector = Icons.Filled.Close,
			contentDescription = stringResource(R.string.description_close_button),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.size(20.dp)
		)
	}
}
