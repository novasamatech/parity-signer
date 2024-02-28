package io.parity.signer.domain.storage

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.uniffi.QrData
import org.junit.After
import org.junit.Assert
import org.junit.Before
import org.junit.Test


class ClearCryptedStorageTest {

	private val storage = ClearCryptedStorage()

    @Before
    fun setUp() {
			val context = ServiceLocator.appContext
			storage.init(context)
    }

    @After
    fun tearDown() {
    }

	@Test
	fun savingQrCodes() {
		val seedName = "testname"
		val qrCodes = listOf(QrData.Regular(PreviewData.exampleQRData),
			QrData.Regular(PreviewData.exampleQRData))
		storage.saveBsQRCodes(seedName, qrCodes)
		val recovered = storage.getBsQrCodes(seedName)

		Assert.assertEquals(qrCodes.size, recovered!!.size)
		Assert.assertEquals(qrCodes, recovered)
	}
}
