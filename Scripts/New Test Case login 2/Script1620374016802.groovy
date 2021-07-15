import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

for (def row = 1; row <= findTestData('New Test Data login 1').getRowNumbers(); row++) {
    WebUI.openBrowser('')

    WebUI.navigateToUrl('http://localhost:4200/')

    WebUI.click(findTestObject('Object Repository/Test case login 2/Page_React App/button_ng nhp'))

    WebUI.setText(findTestObject('Object Repository/Test case login 2/Page_React App/input_ng nhp_email'), findTestData(
            'New Test Data login 1').getValue('Account', row), FailureHandling.STOP_ON_FAILURE)

    WebUI.setText(findTestObject('Object Repository/Test case login 2/Page_React App/input_ng nhp_password'), findTestData(
            'New Test Data login 1').getValue('Password', row), FailureHandling.STOP_ON_FAILURE)

    WebUI.click(findTestObject('Object Repository/Test case login 2/Page_React App/button_ng nhp_1'))

    WebUI.closeBrowser()
}

