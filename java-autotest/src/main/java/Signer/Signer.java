//package Signer;
//
//import java.nio.charset.StandardCharsets;
//import java.security.MessageDigest;
//import java.security.NoSuchAlgorithmException;
//
//public class Signer {
//
//    public static String sign(String message, String privateKey) {
//
//        ECKey.ECDSASignature signature = null;
//        try {
//            ECKey key = ECKey.fromPrivate(TypeConverter.hexStringToByteArray(privateKey));
//            signature = key.doSign(MessageDigest.getInstance("SHA-256")
//                            .digest(message.getBytes(StandardCharsets.UTF_8)));
//        } catch (NoSuchAlgorithmException e) {
//            e.printStackTrace();
//        }
//
//        if (null == signature)
//            return null;
//
//        String signedMessage = signature.toHex();
//
//        return signedMessage.substring(2, signedMessage.length() - 2);
//    }
//}
