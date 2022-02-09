package io.parity.signer.components

import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*

@Composable
fun SeedBox(seedPhrase: String, status: SeedBoxStatus = SeedBoxStatus.Seed) {
	Surface(
		shape = MaterialTheme.shapes.large,
		color = when (status) {
			SeedBoxStatus.Seed -> MaterialTheme.colors.Crypto100
			SeedBoxStatus.Timeout -> MaterialTheme.colors.Bg200
			SeedBoxStatus.Error -> MaterialTheme.colors.BgDanger
			SeedBoxStatus.Locked -> MaterialTheme.colors.BgDanger
		},
		modifier = Modifier.padding(8.dp)
	) {
		when (status) {
			SeedBoxStatus.Seed -> {
				Text(
					seedPhrase,
					style = CryptoTypography.body1,
					color = MaterialTheme.colors.Crypto400,
					textAlign = TextAlign.Center,
					modifier = Modifier.padding(8.dp)
				)
			}
			SeedBoxStatus.Timeout -> {
				Text(
					"Time out. Come back again to see the seed phrase!",
					style = MaterialTheme.typography.body1,
					color = MaterialTheme.colors.Text300,
					textAlign = TextAlign.Center,
					modifier = Modifier.padding(8.dp)
				)
			}
			SeedBoxStatus.Error -> {
				Text(
					"Seed phrase could not be shown due to error",
					style = MaterialTheme.typography.body1,
					color = MaterialTheme.colors.SignalDanger,
					textAlign = TextAlign.Center,
					modifier = Modifier.padding(8.dp)
				)
			}
			SeedBoxStatus.Locked -> {
				Text(
					"Seeds are not available now! Come back again to access them.",
					style = MaterialTheme.typography.body1,
					color = MaterialTheme.colors.SignalDanger,
					textAlign = TextAlign.Center,
					modifier = Modifier.padding(8.dp)
				)
			}
		}
	}
}


enum class SeedBoxStatus {
	Seed,
	Timeout,
	Error,
	Locked;
}
