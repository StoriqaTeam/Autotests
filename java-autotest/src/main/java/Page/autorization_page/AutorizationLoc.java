package Page.autorization_page;

public enum AutorizationLoc {
    SIGNIN("//*[@data-test='headerSignInButton']"),
    SIGNUP("//*[@data-test='headerSignUpButton']"),
    FIRSTNAME("//*[@data-test='firstName']"),
    LASTNAME("//*[@data-test='lastName']"),
    LOGIN("//*[@data-test='email']"),                            //поле Login
    PASSWORD("//*[@data-test='password']"),                      //поле Password
    TERMS("//*[@data-test='terms']"),
    PRIVASY("//*[@data-test='privacy']"),
    ENTER("//*[@data-test='signInButton']");                    //кнопка "Войти"

    private final String cl;

    AutorizationLoc(String cl){
        this.cl = cl;
    }

    public String getCl(){
        return cl;
    }
}
