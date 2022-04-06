package io.parity.signer.modals

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ShieldAlert
import io.parity.signer.components.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.decode64
import io.parity.signer.models.getSeed
import io.parity.signer.models.toListOfJSONObjects
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.modal
import kotlinx.coroutines.delay
import org.json.JSONObject

/**
 * Modal to show seed phrase. Dangerous place.
 * TODO: make sure seed phrase is cleared at all times!!!
 */
@Composable
fun SeedBackup(
	modalData: State<JSONObject?>,
	getSeedForBackup: (String, (String) -> Unit, (SeedBoxStatus) -> Unit) -> Unit,
) {
	val seedName = modalData.value?.optString("seed_name") ?: ""
	var seedPhrase by remember { mutableStateOf("") }
	var seedBoxStatus by remember { mutableStateOf(SeedBoxStatus.Locked) }
	val derivations =
		modalData.value?.optJSONArray("derivations")
			?.toListOfJSONObjects()?.sortedBy { it.optInt("network_order") }
			?: listOf()
	val time = remember { mutableStateOf(60000L) } //Countdown time

	Surface(
		color = MaterialTheme.colors.Bg200,
		shape = MaterialTheme.shapes.modal
	) {
		Box {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar("Backup", seedName.decode64())
				HeadingOverline("SEED PHRASE")
				SeedBox(seedPhrase = seedPhrase, status = seedBoxStatus)
				HeadingOverline("DERIVED KEYS")
				LazyColumn {
					for (pack in derivations) {
						item {
							NetworkCard(pack)
						}
						val networkDerivations =
							pack.getJSONArray("id_set")
								.toListOfJSONObjects().sortedBy { it.optString("path") }
						/*
						//TODO: this could have been neat items block,
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
								if (record.optString("path").isBlank()) {
									Text(
										"seed key",
										style = CryptoTypography.body2,
										color = MaterialTheme.colors.Crypto400
									)
								} else {
									Row {
										Text(
											record.optString("path"),
											style = CryptoTypography.body2,
											color = MaterialTheme.colors.Crypto400
										)
										if (record.optBoolean("has_pwd")) {
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
						text = if (time.value > 0) "Hide seed phrase in " + (time.value / 1000L).toString() + "s" else "",
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
		getSeedForBackup(seedName, {seedPhrase = it}, {seedBoxStatus = it})
		onDispose { seedPhrase = "" }
	}
}
