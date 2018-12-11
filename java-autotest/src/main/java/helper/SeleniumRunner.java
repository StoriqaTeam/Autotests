package helper;

import org.junit.runner.notification.RunNotifier;
import org.junit.runners.BlockJUnit4ClassRunner;
import org.junit.runners.model.InitializationError;

public class SeleniumRunner extends BlockJUnit4ClassRunner {

    private SeleniumRunListener seleniumRunListener;

    public SeleniumRunner(Class klass) throws InitializationError {
        super(klass);
        seleniumRunListener = new SeleniumRunListener();
    }

    public void run(final RunNotifier notifier) {
        notifier.addListener(seleniumRunListener);
        super.run(notifier);
    }
}