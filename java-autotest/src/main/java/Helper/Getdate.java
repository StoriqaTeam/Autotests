package Helper;

import java.text.SimpleDateFormat;
import java.util.Calendar;

public class Getdate {

    public static String Date() {
        return new SimpleDateFormat("yyyyMMddHHmmss").format(Calendar.getInstance().getTime());
    }
}
