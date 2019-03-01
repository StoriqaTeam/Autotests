//package Signer;
//
//public class SignUtil {
//
//    public static String createSignHeader(String json) {
//        String privateKeyHex = "3bbf0cabc0a970b93ffbaedb25db780422fa2a329be47099b3103dee26b8e052";
//        String message = json;
//        return Signer.sign(message, privateKeyHex);
//    }
//
//    public static void main(String[] args) {
//        String timestamp = "" + System.currentTimeMillis();
//        String sign = createSignHeader(timestamp);
//        System.out.println(sign);
//    }
//}