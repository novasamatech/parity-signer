package io.parity.signer.components.exposesecurity

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.zIndex
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkState
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400

@Composable
fun ExposedIcon(
	networkState: State<NetworkState?>,
	onClick: Callback,
	modifier: Modifier = Modifier,
) {
	when (networkState.value) {
		NetworkState.Active -> ExposedIconActive(
			modifier = modifier,
			onClick = onClick,
		)

		NetworkState.Past -> ExposedIconPast(
			modifier = modifier,
			onClick = onClick,
		)

		NetworkState.None, null -> {
			//emty view
		}
	}
}

@Composable
private fun ExposedIconActive(
	modifier: Modifier,
	onClick: Callback,
) {
	Box(
		modifier = modifier
			.zIndex(1f)
			.shadow(
				elevation = 8.dp,
				shape = CircleShape,
			)
			.size(56.dp)
			.background(MaterialTheme.colors.red400, CircleShape)
			.clickable(onClick = onClick),
		contentAlignment = Alignment.Center
	) {
		Image(
			painter = painterResource(id = R.drawable.ic_shield_exposed_32),
			contentDescription = stringResource(R.string.description_shield_exposed_icon),
			colorFilter = ColorFilter.tint(Color.White),
			modifier = Modifier.size(32.dp),
		)
	}
}


@Composable
private fun ExposedIconPast(
	modifier: Modifier,
	onClick: Callback,
) {
	Box(
		modifier = modifier
			.zIndex(1f)
			.shadow(
				elevation = 8.dp,
				shape = CircleShape,
			)
			.size(56.dp)
			.background(MaterialTheme.colors.red400, CircleShape)
			.clickable(onClick = onClick),
		contentAlignment = Alignment.Center
	) {
		Image(
			painter = painterResource(id = R.drawable.ic_wifi_32),
			contentDescription = stringResource(R.string.description_shield_exposed_icon),
			colorFilter = ColorFilter.tint(Color.White),
			modifier = Modifier.size(32.dp),
		)
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
private fun PreviewExposedIcon() {
	SignerNewTheme {
		Column(
			modifier = Modifier.size(300.dp),
		) {
			ExposedIconActive(Modifier, {})
			ExposedIconPast(Modifier, {})
		}
	}
}
