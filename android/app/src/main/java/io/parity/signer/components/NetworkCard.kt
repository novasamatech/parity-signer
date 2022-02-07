package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowCircleDown
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.focusModifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Web3Typography
import org.json.JSONObject

@Composable
fun NetworkCard(network: JSONObject, selected: Boolean = false) {
	Surface(
		shape = MaterialTheme.shapes.large,
		color = MaterialTheme.colors.Bg200,
		modifier = Modifier.height(47.dp)
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier
				.height(36.dp)
				.padding(horizontal = 20.dp)
		) {
			NetworkLogoName(
				logo = network.optString("logo", network.optString("network_logo")),
				name = network.optString("title", network.optString("network_title"))
			)
			Spacer(Modifier.weight(1f))
			if (selected) {
				Icon(
					Icons.Default.Check,
					"this network is selected",
					tint = MaterialTheme.colors.Action400
				)
			}
		}
	}
}
