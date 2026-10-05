import AppKit
import CoreGraphics
import ImageIO
import UniformTypeIdentifiers

func color(_ hex: UInt32, alpha: CGFloat = 1) -> CGColor {
    let red = CGFloat((hex >> 16) & 0xff) / 255
    let green = CGFloat((hex >> 8) & 0xff) / 255
    let blue = CGFloat(hex & 0xff) / 255
    return CGColor(red: red, green: green, blue: blue, alpha: alpha)
}

func iconPNG(size: Int) -> Data {
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    guard let context = CGContext(
        data: nil,
        width: size,
        height: size,
        bitsPerComponent: 8,
        bytesPerRow: 0,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
    ) else {
        fatalError("Unable to create icon bitmap")
    }

    let unit = CGFloat(size)
    context.scaleBy(x: unit, y: unit)
    context.setAllowsAntialiasing(true)
    context.setShouldAntialias(true)

    let background = CGPath(
        roundedRect: CGRect(x: 0.025, y: 0.025, width: 0.95, height: 0.95),
        cornerWidth: 0.23,
        cornerHeight: 0.23,
        transform: nil
    )
    context.addPath(background)
    context.setFillColor(color(0x111b35))
    context.fillPath()

    let mane = CGMutablePath()
    mane.move(to: CGPoint(x: 0.43, y: 0.77))
    mane.addCurve(to: CGPoint(x: 0.62, y: 0.96), control1: CGPoint(x: 0.48, y: 0.92), control2: CGPoint(x: 0.58, y: 0.94))
    mane.addLine(to: CGPoint(x: 0.61, y: 0.79))
    mane.addCurve(to: CGPoint(x: 0.83, y: 0.86), control1: CGPoint(x: 0.70, y: 0.87), control2: CGPoint(x: 0.77, y: 0.87))
    mane.addCurve(to: CGPoint(x: 0.70, y: 0.59), control1: CGPoint(x: 0.80, y: 0.73), control2: CGPoint(x: 0.78, y: 0.66))
    mane.addLine(to: CGPoint(x: 0.48, y: 0.52))
    mane.closeSubpath()
    context.addPath(mane)
    context.setFillColor(color(0x55d6c2))
    context.fillPath()

    let horse = CGMutablePath()
    horse.move(to: CGPoint(x: 0.21, y: 0.29))
    horse.addCurve(to: CGPoint(x: 0.32, y: 0.49), control1: CGPoint(x: 0.13, y: 0.35), control2: CGPoint(x: 0.21, y: 0.46))
    horse.addCurve(to: CGPoint(x: 0.40, y: 0.69), control1: CGPoint(x: 0.34, y: 0.57), control2: CGPoint(x: 0.39, y: 0.64))
    horse.addLine(to: CGPoint(x: 0.42, y: 0.89))
    horse.addLine(to: CGPoint(x: 0.52, y: 0.77))
    horse.addCurve(to: CGPoint(x: 0.66, y: 0.73), control1: CGPoint(x: 0.59, y: 0.80), control2: CGPoint(x: 0.64, y: 0.79))
    horse.addCurve(to: CGPoint(x: 0.74, y: 0.56), control1: CGPoint(x: 0.75, y: 0.68), control2: CGPoint(x: 0.75, y: 0.62))
    horse.addCurve(to: CGPoint(x: 0.63, y: 0.39), control1: CGPoint(x: 0.73, y: 0.49), control2: CGPoint(x: 0.70, y: 0.42))
    horse.addCurve(to: CGPoint(x: 0.54, y: 0.29), control1: CGPoint(x: 0.59, y: 0.37), control2: CGPoint(x: 0.56, y: 0.33))
    horse.addLine(to: CGPoint(x: 0.49, y: 0.18))
    horse.addLine(to: CGPoint(x: 0.40, y: 0.30))
    horse.addCurve(to: CGPoint(x: 0.21, y: 0.29), control1: CGPoint(x: 0.32, y: 0.22), control2: CGPoint(x: 0.25, y: 0.23))
    horse.closeSubpath()
    context.addPath(horse)
    context.setFillColor(color(0xf4f1e8))
    context.fillPath()

    context.addEllipse(in: CGRect(x: 0.30, y: 0.51, width: 0.055, height: 0.055))
    context.setFillColor(color(0xffbd59))
    context.fillPath()

    guard let image = context.makeImage() else {
        fatalError("Unable to render icon")
    }
    let data = NSMutableData()
    guard let destination = CGImageDestinationCreateWithData(
        data,
        UTType.png.identifier as CFString,
        1,
        nil
    ) else {
        fatalError("Unable to encode PNG icon")
    }
    CGImageDestinationAddImage(destination, image, nil)
    guard CGImageDestinationFinalize(destination) else {
        fatalError("Unable to finish PNG icon")
    }
    return data as Data
}

func appendUInt32(_ value: UInt32, to data: inout Data) {
    var littleEndian = value.littleEndian
    withUnsafeBytes(of: &littleEndian) { data.append(contentsOf: $0) }
}

guard CommandLine.arguments.count == 2 else {
    fputs("usage: generate-desktop-icons.swift ICON_ASSET_DIRECTORY\n", stderr)
    exit(2)
}

let output = URL(fileURLWithPath: CommandLine.arguments[1], isDirectory: true)
try FileManager.default.createDirectory(at: output, withIntermediateDirectories: true)

let linuxPNG = iconPNG(size: 512)
try linuxPNG.write(to: output.appendingPathComponent("rocinante.png"))

let windowsPNG = iconPNG(size: 256)
var windowsICO = Data([0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 32, 0])
appendUInt32(UInt32(windowsPNG.count), to: &windowsICO)
appendUInt32(22, to: &windowsICO)
windowsICO.append(windowsPNG)
try windowsICO.write(to: output.appendingPathComponent("Rocinante.ico"))
