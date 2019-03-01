//package Helper;
//
//import Signer.SignUtil;
//import com.google.gson.JsonObject;
//import org.apache.http.HttpResponse;
//import org.apache.http.client.HttpClient;
//import org.apache.http.client.methods.HttpPost;
//import org.apache.http.entity.StringEntity;
//import org.apache.http.impl.client.HttpClientBuilder;
//
//import java.util.UUID;
//
//public class Payment {
//
//    public static final void simulatePayment(String amount, String currency) {
//        HttpClient httpClient = HttpClientBuilder.create().build(); //Use this instead
//
////        String timestamp = "" + System.currentTimeMillis();
////        String deviceId = "65675456565676789879879879";
////        String authorization = "Bearer " + "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1c2VyX2lkIjoxNjYsImV4cCI6MTU0NjAyMzEzOCwicHJvdmlkZXIiOiJFbWFpbCJ9.QSf4j2RCbiJgCIr7PtVrMQX8pVHxQs8n1nWUCZ8Y5hWpaGzooWrLy-2tI11yYVf4TPx12nCP3Z9XriqYZNmKneeHXE9n5-CqjTyMtnfRyB18wxEFzwkVN3u3tfcudEemNYAX6GqUghKp7fLPjzTnwLW4rtw56SC60wLBQvxbZXY8dtfv-SuxuNebNC4h4v5A4oLrlhRmQpujpah8igyK57BcIlYWkEhe70i8pXQVBuIC4F_86VRWV44WKZZmuiR-JFwnORV1rBEXV7b5pf60FntfwImDV2AzKFq64PBxVEXmOA4Um1nN0L0mQNPwCu7AgssAUxorXLyVLHOjvBiETg";
////        String sign = SignUtil.createSignHeader(timestamp, deviceId);
//
//        JsonObject jsonObj = new JsonObject();
//        jsonObj.addProperty("url", "");
//        jsonObj.addProperty("transactionId", UUID.randomUUID().toString());
//        jsonObj.addProperty("amountCaptured", amount);
//        jsonObj.addProperty("currency", currency);
//        jsonObj.addProperty("address", "0xbf6bd6fb7f3b42ac52e08b9b51b5faa789284618");
//        jsonObj.addProperty("accountId", (String) null);
//
//        String json = jsonObj.toString();
//        String sign = SignUtil.createSignHeader(json);
//
//        try {
//            HttpPost request = new HttpPost("https://stage.stq.cloud/v2/callback/payments/inbound_tx");
//            StringEntity params = new StringEntity(json);
////            request.addHeader("content-type", "application/x-www-form-urlencoded");
//            request.addHeader("content-type", "application/json");
////            request.addHeader("authorization", authorization);
////            request.addHeader("timestamp", timestamp);
////            request.addHeader("device-id", deviceId);
//            request.addHeader("sign", sign);
//            request.setEntity(params);
//            HttpResponse response = httpClient.execute(request);
//
//            System.out.println(response);
//
//            if (response.getStatusLine().getStatusCode() != 200)
//                throw new Exception("Response is not OK. Code:" + response.getStatusLine().getStatusCode() + ". " + response.getStatusLine().getReasonPhrase());
//
//            System.out.println();
//        } catch (Exception ex) {
//            System.out.println(ex.getMessage());
//        }
//    }
//
//    public static void main(String[] args) {
//        simulatePayment("60000000000000000000000", "stq");
//    }
//
//}
