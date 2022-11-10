package io.parity.signer.screens.scan.items

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.fill30
import io.parity.signer.ui.theme.pink500


@Composable
fun CameraMultiSignIcon(
	isEnabled: Boolean,
	modifier: Modifier = Modifier,
	onClick: Callback,
) {
	Box(
		modifier = modifier
			.size(32.dp)
			.clickable(onClick = onClick)
			.background(
				if (isEnabled) Color.White else MaterialTheme.colors.fill30, CircleShape
			),
		contentAlignment = Alignment.Center,
	) {
		Image(
			painterResource(id = R.drawable.ic_filter_none),
			contentDescription = stringResource(R.string.description_close_button),
			colorFilter = ColorFilter.tint(
				if (isEnabled) MaterialTheme.colors.pink500
				else MaterialTheme.colors.primary
			),
			modifier = Modifier.size(20.dp)
		)
	}
}
