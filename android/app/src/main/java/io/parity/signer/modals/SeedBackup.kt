package io.parity.signer.modals

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.*
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.MBackup
import io.parity.signer.uniffi.MscNetworkInfo
import kotlinx.coroutines.delay

/**
 * Modal to show seed phrase. Dangerous place.
 */
@Composable
fun SeedBackup(
	backup: MBackup,
	getSeedForBackup: (String, (String) -> Unit, (SeedBoxStatus) -> Unit) -> Unit
) {
	val seedName = backup.seedName
	var seedPhrase by remember { mutableStateOf("") }
	var seedBoxStatus by remember { mutableStateOf(SeedBoxStatus.Locked) }
	val derivations = backup.derivations.sortedBy { it.networkOrder }
	val time = remember { mutableStateOf(60000L) } // Countdown time in ms

	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.modal
	) {
		Box {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar("Backup", seedName)
				HeadingOverline("SEED PHRASE")
				SeedBox(seedPhrase = seedPhrase, status = seedBoxStatus)
				HeadingOverline("DERIVED KEYS")
				LazyColumn {
					for (pack in derivations) {
						item {
							NetworkCard(
								MscNetworkInfo(
									networkTitle = pack.networkTitle,
									networkLogo = pack.networkLogo
								)
							)
						}
						val networkDerivations = pack.idSet.sortedBy { it.path }
						/*
						// TODO: this could have been neat items block,
						//but passworded keys might collide
						//
						//add this to revert:
						//import androidx.compose.foundation.lazy.items
						this.items(
							items = networkDerivations,
							key = {
								pack.optString("network_order") + it.optString("seed") + it.optString("path") + it.optBoolean(
									"has_pwd"
								).toString()
							}
						)*/
						for (record in networkDerivations) {
							item {
								if (record.path.isBlank()) {
									Text(
										"seed key",
										style = CryptoTypography.body2,
										color = MaterialTheme.colors.Crypto400
									)
								} else {
									Row {
										Text(
											record.path,
											style = CryptoTypography.body2,
											color = MaterialTheme.colors.Crypto400
										)
										if (record.hasPwd) {
											Text(
												"///",
												style = CryptoTypography.body2,
												color = MaterialTheme.colors.Crypto400
											)
											Icon(
												Icons.Default.Lock,
												"Password protected",
												tint = MaterialTheme.colors.Crypto400
											)
										}
										Spacer(Modifier.weight(1f))
									}
								}
							}
						}
					}
				}
			}
			if (seedBoxStatus == SeedBoxStatus.Seed) {
				Column(
					verticalArrangement = Arrangement.Bottom,
					modifier = Modifier.fillMaxSize()
				) {
					BigButton(
						text = if (time.value > 0)
							"Hide seed phrase in " + (time.value / 1000L).toString() + "s"
						else "",
						action = {
							seedBoxStatus = SeedBoxStatus.Timeout
							seedPhrase = ""
						}
					)
					Spacer(Modifier.height(20.dp))
				}
			}
		}
	}

	LaunchedEffect(key1 = time.value, key2 = seedBoxStatus) {
		if (seedBoxStatus == SeedBoxStatus.Seed) {
			if (time.value > 0) {
				delay(1000L)
				time.value -= 1000L
			} else {
				seedBoxStatus = SeedBoxStatus.Timeout
			}
		}
	}

	DisposableEffect(Unit) {
		getSeedForBackup(seedName, { seedPhrase = it }, { seedBoxStatus = it })
		onDispose { seedPhrase = "" }
	}
}
