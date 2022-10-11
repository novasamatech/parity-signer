//
//  CameraVideoOutputDelegate.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/10/2022.
//

import AVKit
import Vision

protocol QRPayloadUpdateReceiving: AnyObject {
    func didReceive(update qrCodePayload: String)
}

final class CameraVideoOutputDelegate: NSObject, AVCaptureVideoDataOutputSampleBufferDelegate {
    private weak var updateReceiver: QRPayloadUpdateReceiving!

    func captureOutput(_: AVCaptureOutput, didOutput sampleBuffer: CMSampleBuffer, from _: AVCaptureConnection) {
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            print("Failed to obtain pixelbuffer for this frame")
            return
        }

        let imageRequestHandler = VNImageRequestHandler(cvPixelBuffer: pixelBuffer, options: [:])

        do {
            let detectionRequest = VNDetectBarcodesRequest(completionHandler: qrCodeDetection)
            detectionRequest.symbologies = [.qr]
            try imageRequestHandler.perform([detectionRequest])
        } catch {
            print("Failed to handle \(error)")
        }
    }

    func set(updateReceiver: QRPayloadUpdateReceiving) {
        self.updateReceiver = updateReceiver
    }
}

private extension CameraVideoOutputDelegate {
    func qrCodeDetection(request: VNRequest, error: Error?) {
        if error != nil {
            print("QR code detection error: \(String(describing: error))")
        }

        guard
            let qrCodeDescriptor = (request as? VNDetectBarcodesRequest)?.results?.first?
            .barcodeDescriptor as? CIQRCodeDescriptor
        else { return }

        let qrCodePayload = qrCodeDescriptor.errorCorrectedPayload.map { String(format: "%02x", $0) }.joined()
        updateReceiver.didReceive(update: qrCodePayload)
    }
}
