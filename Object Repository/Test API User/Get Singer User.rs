<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Singer User</name>
   <tag></tag>
   <elementGuidId>6356633c-3703-4d0f-b79d-9ad54793b2b7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users/2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'data.first_name', &quot;Janet&quot;)
WS.verifyElementPropertyValue(response, 'data.last_name', &quot;Weaver&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'support.text', &quot;To keep ReqRes free, contributions towards server costs are appreciated!&quot;)
WS.verifyElementPropertyValue(response, 'data.first_name', &quot;Janet&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
