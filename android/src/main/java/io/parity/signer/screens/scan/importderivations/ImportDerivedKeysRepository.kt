package io.parity.signer.screens.scan.importderivations

import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.storage.mapError
import io.parity.signer.uniffi.DdDetail
import io.parity.signer.uniffi.DerivedKeyStatus
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.SeedKeysPreview
import io.parity.signer.uniffi.importDerivations
import io.parity.signer.uniffi.populateDerivationsHasPwd


class ImportDerivedKeysRepository(
	private val seedRepository: SeedRepository,
) {

	fun importDerivedKeys(seedKeysPreview: List<SeedKeysPreview>): RepoResult<Unit> {
		val newSeeds = seedKeysPreview.map {
			it.copy(derivedKeys = it.derivedKeys
				.filter { key -> key.status == DerivedKeyStatus.Importable })
		}
		return try {
			importDerivations(newSeeds)
			RepoResult.Success(Unit)
		} catch (e: java.lang.Exception) {
			RepoResult.Failure(e)
		}
	}

	suspend fun updateWithSeed(seedPreviews: List<SeedKeysPreview>): RepoResult<List<SeedKeysPreview>> {
		val seeds: Map<String, String> =
			seedRepository.getAllSeeds().mapError() ?: return RepoResult.Failure()
		return try {
			val filledSeedKeys = populateDerivationsHasPwd(seeds, seedPreviews)
			RepoResult.Success(filledSeedKeys)
		} catch (e: java.lang.Exception) {
			RepoResult.Failure(e)
		}
	}

	fun createDynamicDerivationKeys(
		seedName: String,
		seedPhrase: String,
		keysToImport: List<DdDetail>,
	): OperationResult<Unit, ImportDerivedKeyError > {
		val occuredErrors = listOf<Pair<String, String>>()
		keysToImport.forEach { key ->
			try {

			} catch (e: ErrorDisplayed) {

			} catch (e: Error) {

			}
		}

//		 let result: Result<Void, ImportDerivedKeyError>
//            keysToImport.forEach {
//                do {
//                    try tryCreateImportedAddress(
//                        seedName: seedName,
//                        seedPhrase: seedPhrase,
//                        path: $0.path,
//                        network: $0.networkSpecsKey
//                    )
//                } catch let displayedError as ErrorDisplayed {
//                    occuredErrors.append((key: $0.path, error: displayedError.localizedDescription))
//                } catch {
//                    occuredErrors.append((key: $0.path, error: error.localizedDescription))
//                }
//            }
//            if occuredErrors.isEmpty {
//                result = .success(())
//            } else if occuredErrors.count == keysToImport.count {
//                result = .failure(.noKeysImported(errors: occuredErrors.map(\.error)))
//            } else {
//                result = .failure(.keyNotImported(occuredErrors))
//            }
//            self.callbackQueue.async {
//                completion(result)
//            }
	}

	sealed class ImportDerivedKeyError {
		abstract val description: String
		data class NoKeysImported(val errors: List<String>): ImportDerivedKeyError() {
			override val description: String
				get() = errors.joinToString(separator = "\n")
		}

		data class KeyNotImported(val keyToError: List<Pair<String, String>>): ImportDerivedKeyError() {
			override val description: String
				get() = TODO("Not yet implemented")
//			todo dmitry
//			         return errorInfo.reduce(into: "") {
//                $0 += (
//                    Localizable.AddDerivedKeys.Error
//                        .DerivedKeyForNetwork.content($1.key, $1.error) + "\n"
//                )
//            }
		}
	}

}
