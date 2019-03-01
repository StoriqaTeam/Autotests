package Helper;

import org.junit.internal.AssumptionViolatedException;
import org.junit.runner.Description;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;
import org.junit.runner.notification.RunListener;

public class SeleniumRunListener extends RunListener {
    /**
     * Вызывается перед запуском всех тестов.
     * @param description описание класса, который будет запущен
     */
    @Override
    public void testRunStarted(Description description) throws Exception {
        System.out.println("Before tests run: " + description);
    }

    /**
     * Вызывается когда все тесты завершены
     * @param result результат выполнения тестов
     */
    @Override
    public void testRunFinished(Result result) throws Exception {
        System.out.println("Result of the test run:");
        System.out.println("Run time: " + result.getRunTime() + " ms");
        System.out.println("Run count: " + result.getRunCount());
        System.out.println("Failure count: " + result.getFailureCount());
        System.out.println("Ignored count: " + result.getIgnoreCount());
    }

    /**
     * Вызывается перед запуском каждого теста.
     * @param description описание теста, который собирается запуститься
     * (обычно имя класса и метода)
     */
    @Override
    public void testStarted(Description description) throws Exception {
        System.out.println("Test starts: " + description);
    }

    /**
     * Вызывается после завершения каждого теста,
     * несмотря на результат выполнения.
     * @param description описание теста, который завершился
     */
    @Override
    public void testFinished(Description description) throws Exception {
        System.out.println("Test finished: " + description);
        System.out.println("--------------------------------------");
    }

    /**
     * Вызывается когда тест завершается неудачей.
     * @param failure описывает тест, который завершился ошибкой
     * и полученное исключение.
     */
    @Override
    public void testFailure(Failure failure) throws Exception {
        System.out.println("Test failed with: "
                + failure.getException());
    }

    /**
     * Вызывается когда не выполняется условие в классе Assume
     *
     * @param failure описывает тест, который не был выполнен
     * и исключение {@link AssumptionViolatedException}
     */
    @Override
    public void testAssumptionFailure(Failure failure) {
        System.out.println("Test assumes: " + failure.getException());
    }

    /**
     * Вызывается когда тест не будет запущен,
     * в основном потому что стоит аннотация @Ignore
     *
     * @param description описание теста который не будет запущен
     */
    @Override
    public void testIgnored(Description description) throws Exception {
        System.out.println("Test ignored: " + description);
        System.out.println("--------------------------------------");
    }
}