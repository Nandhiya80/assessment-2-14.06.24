<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_SB Collect       Home    TransactionHi_67028f_1</name>
   <tag></tag>
   <elementGuidId>d60841cf-2dbe-4063-8f92-d33e091c0a2f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>body</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>25f8a7bd-044d-49f7-a961-9611e9665f0e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onload</name>
      <type>Main</type>
      <value>myFunction1()</value>
      <webElementGuid>48e3298d-b85b-4d62-a4da-6b17f98b89ed</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
	
		

		
		
		
			




  

  
  
	 	
	
		  
	
	 	
	
		
	
	
				 
			 
				
					 SB Collect 
				
				
								
			 
			
			 
			

				
				
				
				
				  
					
					  Home 
					
					
					   Transaction
									History
							
					
					  FAQ's
					
					
					  Customer Support
					
				  
				

			
			
			 
	
		
	
	
  
  

 

		
		
		
			












 


SB Collect












#loginAudioCaptcha{
	width:100% !important;
}
#refreshImgCaptcha
{
width:125px;
 margin-top: -8px;
}

@media only screen 
and (min-device-width : 768px) 
and (max-device-width : 1024px)  {
#refreshImgCaptcha
{
width:120px !important;
}
}
 
 
 
 $(document).ready(function() {
		getUserSelImgCaptcha();
		
});

	// Opera 8.0+
 var isOpera = (!!window.opr &amp;&amp; !!opr.addons) || !!window.opera || navigator.userAgent.indexOf(' OPR/') >= 0;
 // Firefox 1.0+
	var isFirefox = typeof InstallTrigger !== 'undefined';
 // Safari 3.0+ &quot;[object HTMLElementConstructor]&quot; 
 var isSafari = /constructor/i.test(window.HTMLElement) || (function (p) { return p.toString() === &quot;[object SafariRemoteNotification]&quot;; })(!window['safari'] || (typeof safari !== 'undefined' &amp;&amp; safari.pushNotification));
 // Internet Explorer 6-11
 var isIE = /*@cc_on!@*/false || !!document.documentMode;
 // Edge 20+
 var isEdge = !isIE &amp;&amp; !!window.StyleMedia;
 // Chrome 1 - 71
 var isChrome = !!window.chrome &amp;&amp; (!!window.chrome.webstore || !!window.chrome.runtime);
 // Blink engine detection
 var isBlink = (isChrome || isOpera) &amp;&amp; !!window.CSS;
	//window.onload = function(){ 
	 
		//getUserSelImgCaptcha();
		
//	} 
  
  function getUserSelImgCaptcha() {
 	
 	if(null!=document.paymentDetails.captchaValue){//audio
 		document.paymentDetails.captchaValue.value = ''; //audio
     	document.paymentDetails.capOption.value = 'IMG';
     //	document.getElementById('noAudImgSelection').style.display = 'none';
 		document.getElementById('imgselection').style.display = 'block';
 		document.getElementById('imgselcaptcha').style.display = 'block';
 		document.getElementById('audioselection').style.display = 'none';
 		document.getElementById('audelcaptcha').style.display = 'none';
 		document.getElementById(&quot;refreshImgCaptcha&quot;).src='/sbicollect/simpleCaptchaServ?'+(new Date().getTime());

 	}
 	
	}


 function getUserSelAudCaptcha() {
 	document.paymentDetails.captchaValue.value = '';
 	document.paymentDetails.capOption.value = 'AUD';
 	//document.getElementById('noAudImgSelection').style.display = 'none';
 	document.getElementById('imgselection').style.display = 'none';
		document.getElementById('imgselcaptcha').style.display = 'none';
		document.getElementById('audioselection').style.display = 'block';
		document.getElementById('audelcaptcha').style.display = 'block';
		if (isIE == true){
			document.getElementById('IEAud').style.display = 'block';
			document.getElementById('othrIEAud').style.display = 'none';
		}else {
			document.getElementById('othrIEAud').style.display = 'block';
			document.getElementById('IEAud').style.display = 'none';
			document.getElementById(&quot;loginAudioCaptcha&quot;).src = '/sbicollect/audio.wav?bogus=' + new Date().getTime();
				
		}
	}
 
 function refreshImg()
 {
 	if(null!=document.getElementById(&quot;refreshImgCaptcha&quot;)){
 document.getElementById(&quot;refreshImgCaptcha&quot;).src='/sbicollect/simpleCaptchaServ?'+(new Date().getTime());
 
 	}
 }   
 
 



	
		 
		 
			 
			 
			
		 
		
			 
			 
			 
			
		    
		    
		
		
		 
		 
		 
		 
		
		
		
		
 
		 
		

		
		 
		 
		
		

		
	    


		 
		  
		
		
		 
		
	
             
        
        
        
        
        
      






		
		 
		
			
				
				
					Payment Progress
					
						Select Payee
						Enter Payment Details
						Verify Payment Details
						Complete Payment
						Print Receipt
					
				
			
		
		
		
			
				 
				
				 BDU CHIEF HOSTEL ADMINISTRATOR  |  CENTRAL OFFICE FOR HOSTEL ADMINISTRATION, BHARATHIDASAN UNIVERSITY, , TRICHY-620024
				
	
			
			

			
				


					
						
							Enter Payment Details
							
								
									
									
									
									Payment Category*:
									
									
								
								
								
								
								
									 
									
									
										 --Select any Category-- 
										
											
												

													GUEST FEE
												
												
											
										
											
												
												

													
														HOSTEL ADMISSION FEE 
												
											
										
											
												
												

													
														HOSTEL MESS FEES 
												
											
										
									GUEST FEE
									
									
								
							
						
	

          
          
          	
			
									
								
			
			
				
				
				
				
				
				
				
		  	
			
						
					
						
					
						
						
							
							



NAME *










						
							
							



COURSE/PURPOSE *










						
							
							



DEPARTMENT *










						
							
							



NUMBER OF DAYS *










						
							
							



FROM (DD/MM/YYYY) *










						
							
							



TO (DD/MM/YYYY) *










						
							
							



MOBILE *










						
							
							



AMOUNT (In Rs,) *










						
						

						
							


								
									Remarks : 
								
								
									 
								
							
						
					
				
					
				
				
					Please confirm the detail before proceeding for payment. Save a copy of e-receipt for your future reference.
				
				
					For any queries please contact Office 0431-2407038
				
				
				


						
							Enter Your Details

							
								
									 Individual
								
								
									 Organisation
										/ Corporate
								
							
						



						
							
								
									
										Name * :
										
									
									
										
									
									
									
									
										
										Date of
											Birth * :
										
										
										 
										
									
										
									
									
										
											
											
										 
									
								
							
							
								
									
										Mobile No * :
										
									
									
										
										On successful completion of
											payment,you will receive the transaction reference number on
											this mobile number
									
									
									
										Email ID :
										
									
									
										
										On successful completion of payment,
											you will receive the transaction reference number on this
											email ID
									
								
							

						

						

							

								
									Your Name * :
									
								
								
									
								

								


								
									Organisation
										Name * :
									
								
								
									
								


							
							

								
								
									
									Date of
										Incorporation * :
									
									
									
									
								
										
								
								

									
										 
									
								

								


								
									Mobile No * :
									
								
								
									
									On successful completion of payment,you
										will receive the transaction reference number on this mobile
										number
								


							
							

								
									Email Id :
									
								
								
									
									On successful completion of payment,
										you will receive the transaction reference number on this
										email ID

								

								

							

						
				

						
							
								
									
										 I
											have read and agreed to the Terms
												&amp; Conditions

										
									
								
							
						
					


						
						
							
								
									Enter the text as shown in the
											image
										*:
									

								
								
									Enter the text as from audio *

								
								
								
									
										Select one of the Captcha options
											*
									
                                        
											Image
											Captcha
										
										 
											Audio
											Captcha
										
									

								
								
						
									 
											
											
												
											
								
								
									
											
											
										
										
										
										  
										
								
								
							
						
				
						
							
								
								
								Back
								
								
								
								
								Reset
								
								Next
								
								
							
						
					

				
			
			
			
		
		

		
			
				
					

					
					
						X
						Terms and Conditions
						  
						 
						
								Corporate Customer: Firm/Company/Institution (F/C/I) collecting payment from their beneficiaries.
								User: The beneficiary making a payment to F/C/I for the services/goods availed.
								Bank shall not be responsible, in any way, for the quality or merchantability of any product/merchandise or any of the services related thereto, whatsoever, offered to the User by the Corporate Customer. Any disputes regarding the same or delivery of the Service or otherwise will be settled between Corporate Customer and the User and Bank shall not be a party to any such dispute. Any request for refund by the User on any grounds whatsoever should be taken up directly with the Corporate Customer and the Bank will not be concerned with such a request.
								Bank takes no responsibility in respect of the services provided and User shall not be entitled to make any claim against the Bank for deficiency in the services provided by the Corporate Customer.
								The User shall not publish, display, upload or transmit any information prohibited under Rule 3(2) of the Information Technology (Intermediaries guidelines) Rules, 2011.
								In case of non-compliance of the terms and conditions of usage by the User, the Bank has the right to immediately terminate the access or usage rights of the User to the computer resource of the Bank and remove the non-compliant information.
							
					
					
					
						Ok
					
				
			
		

		
	








         $(document).ready(function(){
        	// $(function() {  
        	 //$.datepicker.setDefaults({
        		//   inline: true,
        		 //  showOn: 'button',
        		  // buttonImageOnly: true,
        		  // buttonImage: '/sbijava/sbcollectrevamp/images/calendar_icon.png',
        		 //  buttonText: 'Calendar'
        		  
        		//   }); 
        	// $(&quot;#dateOfBirth&quot;).datepicker({ 
        		// dateFormat: &quot;dd/mm/yy&quot;,
        		// yearRange: &quot;-150:+0&quot;,
        		    
        		// maxDate: new Date()
        		// }).val()          
        		//  var changeMonth = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot; ); 
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot;, true ); 
        		// var changeYear = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot; );
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot;, true );
        		// $( &quot;#dateOfBirth&quot; ).datepicker({ appendText: &quot;(dd/mm/yy)&quot; });
        	// });
        		 
        	 
	   $(&quot;#ForOrganisation&quot;).hide();
	   
        $('input[type=&quot;radio&quot;]').click(function(){
           if($(this).attr(&quot;value&quot;)==&quot;formInd&quot;){
               $(&quot;#ForOrganisation&quot;).hide();    
               $(&quot;#ForIndividual&quot;).show();
              //Added as part of Reset defect
                    $('#cusName12').val('').prop(&quot;removed&quot;, true).focus();
                    $('#orgName').val('').prop(&quot;removed&quot;, true).focus();
                    $(&quot;#dateOfBirth12&quot;).datepicker('setDate', null); 
                    $('#mobileNo12').val('').prop(&quot;removed&quot;, true).focus();
                    $('#emailId12').val('').prop(&quot;removed&quot;, true).focus();
                    $('#captchaValue').val('').prop(&quot;removed&quot;, true).focus();//def-6
                    resetToImageCaptcha();
                                   
                }
          else if ($(this).attr(&quot;value&quot;)==&quot;fileOrg&quot;){
               $(&quot;#ForIndividual&quot;).hide();
              $(&quot;#ForOrganisation&quot;).show();
			   //$('#cusName').find('input').val('');
                   $('#cusName').val('').prop(&quot;removed&quot;, true).focus();
                   $(&quot;#dateOfBirth&quot;).datepicker('setDate', null);
                   $('#mobileNo').val('').prop(&quot;removed&quot;, true).focus();
                   $('#emailId').val('').prop(&quot;removed&quot;, true).focus(); 
                   $('#captchaValue').val('').prop(&quot;removed&quot;, true).focus(); //def-6
                   resetToImageCaptcha();
      }
      });
	   //$(&quot;#exampleModalLong&quot;).modal(&quot;show&quot;);
	  });
        
// Somnath made change starts here
	//$('#dateOfBirth').datepicker({
      //             format:'dd/mm/yyyy',
        //      }).datepicker(&quot;setDate&quot;,'now');
// Somnath made change end here
      //  $('#dateOfBirth12').datepicker({
            //        format:'dd/mm/yyyy',
             //   }).datepicker(&quot;setDate&quot;,'now');
/*         
        $(function() {
        	 $(&quot;#dateOfBirth&quot;).datepicker({  maxDate: new Date() });
         });
         
         
         
         $('#dateOfBirth').datepicker({
            format: &quot;dd/mm/yyyy&quot;,
           	autoclose: true,
           	orientation: &quot;top&quot;,
          	endDate: &quot;today&quot;

       });
         
         
        $(&quot;.dateOfBirth&quot;).datepicker({
            maxDate: &quot;-1d&quot;, 
     		minDate: new Date(2007, 6, 12)
     });
         
*/         
//	Somnath made change starts here 
		[&quot;dateOfBirth&quot;,&quot;dateOfBirth12&quot;].forEach(function(val) { 
			$('#'+val).datepicker({autoclose:true});
		});
// 	Somnath made change starts ends 
         
         function selectCategory(category)
         {
           //alert(&quot;Entered into select category onchange&quot;);
         	var w = document.paymentDetails.Category.selectedIndex;
         	
        			  
         			  
         			 var categoryId1='C592823';
         			 var ins1='IN502269';
         			 var st1='Tamil Nadu';
         	
         			 
         			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
         			document.getElementById(&quot;institutionId&quot;).value=ins1;
         			document.getElementById(&quot;state&quot;).value=st1;
         	
         	
         	var selected_text = document.paymentDetails.Category.options[w].text;
         	
         	if (selected_text == &quot;--Select Category--&quot;) {
        		 return false;
         		 
         	} else {
         		
         		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
         		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
         		         		
         		var fileMode=category.options[category.selectedIndex].getAttribute('fileMode');
         		
         		var paramName=category.options[category.selectedIndex].getAttribute('paramName');
         		
         		var optStr=category.options[category.selectedIndex].getAttribute('optString');
         		
         		var dupPayment = category.options[category.selectedIndex].getAttribute('dupPayment');
         		
                  //API    
         		if(fileMode=='Y' || fileMode=='A')
         		{
         			
         			
         				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
         				var arr = paramName.split(&quot;-&quot;);
         				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
         				
       				
         				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
         				
         				var arr = paramName.split(&quot;-&quot;);
             			
             			         				
         				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
         				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
         				
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateNotMandatory&quot;).value='';
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateMandatory&quot;).value='';  
         				//document.getElementById(&quot;controls&quot;).value='';
         				
         				if(document.getElementById(&quot;formInd&quot;) !=null)
         				document.getElementById(&quot;formInd&quot;).value='';
         				
         				if(document.getElementById(&quot;fileOrg&quot;)!=null)
         				document.getElementById(&quot;fileOrg&quot;).value='';
         				
         				if(document.getElementById(&quot;corporate_1&quot;)!=null) 
         					document.getElementById(&quot;corporate_1&quot;).value='';
         				  
         				var onloadenc='numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~firstkey~secondkey~capchaRadioSel';//img Aud Captcha'
         					var reqData = checkEmpty(onloadenc);
         					var encdynamicdata = getEncryptData(reqData);
        					$(&quot;#enconload&quot;).val(encdynamicdata);
         				
         				
         				
         				
         				
         				   $('#firstkey').removeAttr('name');
         				   $('#secondkey').removeAttr('name');
         				
         				
         				   $('#numberofkeyfield').removeAttr('name');
      		               $('#categoryNameSel').removeAttr('name');
      		              // $('#categoryId').removeAttr('name');
      		               $('#instituteId').removeAttr('name');
         				
         				
      		               $('#institutionId').removeAttr('name');
    		               $('#state').removeAttr('name');
    		               $('#categoryName').removeAttr('name');
    		               $('#fileMode').removeAttr('name');
       				
    		               $('#secondkey').removeAttr('name');
      		               $('#firstkey').removeAttr('name');
      		               $('#transactionName').removeAttr('name');
      		               $('#creditTxnCount').removeAttr('name');
         				
      		             $('#debitBranchCode').removeAttr('name');
    		               $('#netbankingflag').removeAttr('name');
    		               $('#branchflag').removeAttr('name');
    		               $('#creditcardflag').removeAttr('name');
       				
    		               $('#otherdebitflag').removeAttr('name');
      		               $('#encData').removeAttr('name');
      		               $('#dobName').removeAttr('name');
      		               $('#creditBranchCode').removeAttr('name');
         				
      		             $('#paramLen').removeAttr('name');
    		               //$('#corpName').removeAttr('name');
    		               $('#corpAddress').removeAttr('name');
    		               $('#instType').removeAttr('name');
       				
    		               $('#logoPath').removeAttr('name');
      		               $('#logoName').removeAttr('name');
      		               $('#individual').removeAttr('name');
      		               $('#organisation').removeAttr('name');
         				
      		             $('#encStaticData').removeAttr('name');
    		               $('#encdynamicData').removeAttr('name');
    		               //$('#encKey').removeAttr('name');
    		              // $('#encKeyVal').removeAttr('name');
       				
         				
    		               $('#encMis').removeAttr('name');
    		               $('#Category').removeAttr('name');
    		               $('#outref11').removeAttr('name');
    		               $('#controls').removeAttr('name');
       				
    		              // $('#validateNotMandatory').removeAttr('name');
    		               $('#outref12').removeAttr('name');
    		               $('#outref13').removeAttr('name');
    		               //$('#validateMandatory').removeAttr('name');
       				
    		               $('#outref14').removeAttr('name');
    		               $('#transactionRemarks').removeAttr('name');
    		               $('#selectAccountFrom').removeAttr('name');
    		               $('#cusName').removeAttr('name');
       				
    		               $('#dateOfBirth').removeAttr('name');
    		               $('#mobileNo').removeAttr('name');
    		               $('#emailId').removeAttr('name');
    		               $('#cusName12').removeAttr('name');
       				
    		               $('#orgName').removeAttr('name');
    		               $('#dateOfBirth12').removeAttr('name');
    		               $('#mobileNo12').removeAttr('name');
    		               $('#emailId12').removeAttr('name');
       				
    		               
    		               $('#CorporateInternetBanking').removeAttr('name');
    		               $('#captchaValue').removeAttr('name');
    		               $('#capchaRadioSel').removeAttr('name'); //audio image captcha
    		               $('#Ur02H').removeAttr('name');
    		              // $('#emailId12').removeAttr('name');
       				
         				
         				
         				
    				    var url = &quot;/sbicollect&quot;+'/payment/filemode.htm';
         				$(&quot;#paymentDetails&quot;).attr('action', url);
         				$(&quot;#paymentDetails&quot;).submit();
       					return false;
 
         			}
         		else
         			{
         			document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;
         			
         			var onloadencCatList='numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~categoryNameSel~capchaRadioSel';//img Aud Captcha
     					var reqData = checkEmpty(onloadencCatList);
     					var encdynamicdata = getEncryptData(reqData);
    					  $(&quot;#encData&quot;).val(encdynamicdata);
     				
     				   $('#numberofkeyfield').removeAttr('name');
  		               $('#categoryNameSel').removeAttr('name');
  		              $('#categoryId').removeAttr('name');
  		               $('#instituteId').removeAttr('name');
     				
     				
  		               $('#institutionId').removeAttr('name');
		               $('#state').removeAttr('name');
		               $('#categoryName').removeAttr('name');
		               $('#fileMode').removeAttr('name');
   				
		               $('#secondkey').removeAttr('name');
  		               $('#firstkey').removeAttr('name');
  		               $('#transactionName').removeAttr('name');
  		               $('#creditTxnCount').removeAttr('name');
     				
  		             $('#debitBranchCode').removeAttr('name');
		               $('#netbankingflag').removeAttr('name');
		               $('#branchflag').removeAttr('name');
		               $('#creditcardflag').removeAttr('name');
   				
		               $('#otherdebitflag').removeAttr('name');
  		               //$('#encData').removeAttr('name');
  		               $('#dobName').removeAttr('name');
  		               $('#creditBranchCode').removeAttr('name');
     				
  		             $('#paramLen').removeAttr('name');
		               //$('#corpName').removeAttr('name');
		               $('#corpAddress').removeAttr('name');
		               $('#instType').removeAttr('name');
   				
		               $('#logoPath').removeAttr('name');
  		               $('#logoName').removeAttr('name');
  		               $('#individual').removeAttr('name');
  		               $('#organisation').removeAttr('name');
     				
  		               $('#encStaticData').removeAttr('name');
		               $('#encdynamicData').removeAttr('name');
		              // $('#encKey').removeAttr('name');
		              // $('#encKeyVal').removeAttr('name');
   				
     				
		               $('#encMis').removeAttr('name');
		               $('#Category').removeAttr('name');
		               $('#outref11').removeAttr('name');
		               $('#controls').removeAttr('name');
   				
		              $('#validateNotMandatory').removeAttr('name');
		               $('#outref12').removeAttr('name');
		               $('#outref13').removeAttr('name');
		               $('#validateMandatory').removeAttr('name');
   				
		               $('#outref14').removeAttr('name');
		               $('#transactionRemarks').removeAttr('name');
		               $('#selectAccountFrom').removeAttr('name');
		               $('#cusName').removeAttr('name');
   				
		               $('#dateOfBirth').removeAttr('name');
		               $('#mobileNo').removeAttr('name');
		               $('#emailId').removeAttr('name');
		               $('#cusName12').removeAttr('name');
   				
		               $('#orgName').removeAttr('name');
		               $('#dateOfBirth12').removeAttr('name');
		               $('#mobileNo12').removeAttr('name');
		               $('#emailId12').removeAttr('name');
   				
		               
		               $('#CorporateInternetBanking').removeAttr('name');
		               $('#captchaValue').removeAttr('name');
		               $('#Ur02H').removeAttr('name');
		               $('#capchaRadioSel').removeAttr('name');//img Aud Captcha
		               $('#categoryNameSel').removeAttr('name');
         			
         			
         			
         			
         			
         			
 				
         				  
      				
         				var url = &quot;/sbicollect&quot;+'/payment/listcategory.htm';
    				    $(&quot;#paymentDetails&quot;).attr('action', url);
         				$(&quot;#paymentDetails&quot;).submit();
       				    return false; 
         			}
         		return true;
         		} 
         }

       
         
          function validateAndSubmitFeeParams(formName)
           {
       	  if(document.getElementById('validateMandatory') != null) {
        		
               //alert(&quot;inside validateManndatory data is &quot;+document.getElementById('validateMandatory'));
        						var validateMandatory=eval(formName+'.validateMandatory');
        						
        						if(validateMandatory.length==undefined)
        							validateMandatory = new Array(eval(formName+'.validateMandatory'));
        							
       							
        						for(i=0;i&lt;validateMandatory.length;i++) {
        							
        							var validations=validateMandatory[i].value.split('#SEP#');
         							var controlObj = eval(formName+'.'+validations[0]);

        							//Start of Code for Employee and Employer Contribution validation 
        							if(document.getElementById('Category'))
        							{
        								 if(document.getElementById('Category').value =='C005942')	{
        										
        										if(validations[1] =='Employee Contribution'){
        											employerContribution = controlObj.value;
        											employerId = validations[0];
        											}
        										
        										if(validations[1] =='Employer Contribution'){
        											employeeContribution = controlObj.value
        											employeeId=validations[0];
        											}
        										
        										if(validations[1] =='Number of Employees') {
        											
        											noOfEmp=controlObj.value;
        											if(noOfEmp*3 != employerContribution){
        												alert(&quot;Enter correct value for Employee Contribution&quot;);
        												document.getElementById(employerId).value=&quot;&quot;;
        												return false;
        											}
        											
        											if(noOfEmp*6 != employeeContribution){
        												alert(&quot;Enter correct value for Employer Contribution&quot;)
        												document.getElementById(employeeId).value=&quot;&quot;;
        												return false;
        											}
        										}
        								
        								} 
        							}
         					//End of Code for Employee and Employer Contribution validation 
         					
         					if(controlObj.value==''){
         						if(validations[3]!=''&amp;&amp; validations[3]!='null')
         							alert(validations[3]);
         						else{
         					
         							if(controlObj.type.indexOf('select') != -1)
         								alert(&quot;Select &quot; + validations[1]);
         							else
         								alert(&quot;Enter &quot; + validations[1]);
         						}
         						controlObj.focus();
         						return false
         					}
         					
         				  //CR-9184 installment changes start
         	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='blank')
         	            	{
         	                 	alert(&quot;please select payment method&quot;);
         		                return false;
         	                	}
         	                //CR-9184 installment changes end
         	
         					if (validations[2]!=''&amp;&amp; validations[2]!='null')
         					{

         					if(controlObj.value!=''&amp;&amp; controlObj.value.search(validations[2])!=0){
         						if(validations[3]!=''&amp;&amp; validations[3]!='null'){
         							if(validations[2] == '(^[0-9a-zA-Z&amp;_. /@-]{1,75}$)')
         								alert(validations[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validations[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validations[3]); 
         							}
         						}
         						else{
         								alert(&quot;Enter &quot; + validations[1]);
         						}		
         						controlObj.focus();
         						return false
         					}
         					}
         					
         				}  // for loop complete
         				
         				
         			}	
         			
         			if(document.getElementById('validateNotMandatory') != null){
         				var validateNotMandatory=eval(formName+'.validateNotMandatory');
         				
         				
         				if(validateNotMandatory.length==undefined)
         					validateNotMandatory = new Array(eval(formName+'.validateNotMandatory'));
         					
         				for(i=0;i&lt;validateNotMandatory.length;i++){
         					
         					var validationsNotMand =validateNotMandatory[i].value.split('#SEP#');
         					var controlObj = eval(formName+'.'+validationsNotMand[0]);
                 			//23032023 date issue - regex changed to accept date and month in 01 - 09
         					if(controlObj.value =='' &amp;&amp; validationsNotMand[2] != '(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)' &amp;&amp; validationsNotMand[2] != '(^(\\d{1,20})(\\.[0-9][0-9])?$)' &amp;&amp; validationsNotMand[2]!='^(0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](0[1-9]|[1-9]|1[012])[- /.](19|20)\\d\\d$' &amp;&amp;  validationsNotMand[2]!='^(0[1-9]|[1-9]|1[012])[- /.](0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](19|20)\\d\\d$'){
         						alert(validationsNotMand[3]);
         						return false;
         					}
         					if(controlObj.value!=''&amp;&amp; controlObj.value.search(validationsNotMand[2])!=0){
         						if(validationsNotMand[3]!=''&amp;&amp; validationsNotMand[3]!='null')
         						{
         							if(validationsNotMand[2] == '(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)')
         								alert(validationsNotMand[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validationsNotMand[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validationsNotMand[3]); 
         							}
         							controlObj.focus();
         							return false;
         						}
         					}
         					
         					
         				}
         		  	}
         			
         			
         			
         			 var len = document.getElementById(&quot;paramLen&quot;).value;
                    
                     var reqdynamicData = '';
                 	var reqrefData='';
                      for(i=1,k=11;i&lt;=len;i++,k++)
                     	{
                     
                     	 var temp;
                     	
  						if(document.getElementsByName(&quot;outref&quot;+k).length >0){ 
 						 temp = document.getElementsByName(&quot;outref&quot;+k)[0].value; 
 						reqrefData = &quot;outref&quot;+k+&quot;=&quot;+temp+'&amp;';
 						
 						reqdynamicData+=reqrefData;
 					
                     	 } 

                     }
                    
                      
                      if(document.getElementById(&quot;installment&quot;)!=null)
               		{
               		var reqInstallment=document.getElementById(&quot;installment&quot;).value;

               		reqdynamicData+='installment='+reqInstallment+'&amp;';
  					} 

                      
                      
                      var reqtransactionName=document.getElementById(&quot;transactionName&quot;).value; 
                      reqdynamicData+='transactionName='+reqtransactionName+'&amp;';
                        
                     
                      
                      var reqcreditTxnCount=document.getElementById(&quot;creditTxnCount&quot;).value; 
                      reqdynamicData+='creditTxnCount='+reqcreditTxnCount+'&amp;';
                      
                      
                      var reqdebitBranchCode=document.getElementById(&quot;debitBranchCode&quot;).value; 
                      reqdynamicData+='debitBranchCode='+reqdebitBranchCode+'&amp;';
                      
                  
                      var reqcreditBranchCode=document.getElementById(&quot;creditBranchCode&quot;).value; 
                      reqdynamicData+='creditBranchCode='+reqcreditBranchCode+'&amp;';
                  
                      var reqinstallmentParamKey=document.getElementById(&quot;installmentParamKey&quot;).value; 
                      reqdynamicData+='installmentParamKey='+reqinstallmentParamKey+'&amp;';
                  
                      
                      
                                       
                      var reqcategoryId=document.getElementById(&quot;categoryId&quot;).value;
                     
                      reqdynamicData+='categoryId='+reqcategoryId+'&amp;';
                  	 
                      
                      
                      var reqcategoryName=document.getElementById(&quot;categoryName&quot;).value; 
                      reqdynamicData+='categoryName='+reqcategoryName+'&amp;';
                      
                      
                      var reqnetbankingflag=document.getElementById(&quot;netbankingflag&quot;).value; 
                      reqdynamicData+='netbankingflag='+reqnetbankingflag+'&amp;';
                  
                      
                      var reqbranchflag=document.getElementById(&quot;branchflag&quot;).value; 
                      reqdynamicData+='branchflag='+reqbranchflag+'&amp;';
                      
                      
                      var reqcreditcardflag=document.getElementById(&quot;creditcardflag&quot;).value;
                      reqdynamicData+='creditcardflag='+reqcreditcardflag+'&amp;';
                      
                      var reqotherdebitflag=document.getElementById(&quot;otherdebitflag&quot;).value; 
                      reqdynamicData+='otherdebitflag='+reqotherdebitflag+'&amp;';
                    
                          					
                  	
                      var reqcontrols=document.getElementById(&quot;controls&quot;).value; 
                      reqdynamicData+='controls='+reqcontrols+'&amp;';
                     
                      //hidden fields ends
                      var reqparamLen = document.getElementById(&quot;paramLen&quot;).value;
                      reqdynamicData+='paramLen='+reqparamLen+'&amp;';
                      
                      //alert(&quot;reqdynamicData is &quot;+reqdynamicData);
                      //var reqData = checkEmpty(reqdynamicData)
                      var encdynamicdata = getEncryptData(reqdynamicData)
  					  $(&quot;#encdynamicData&quot;).val(encdynamicdata);
                      
                  	
              		 
                     	
                      
                      var e = document.getElementById(&quot;Category&quot;);
                      
                     
                      var strUser = e.options[e.selectedIndex].text;
                      document.getElementById(&quot;categoryName&quot;).value=strUser;
                      
                    
                     
                     var e = document.getElementById(&quot;Category&quot;);
                     var strUser = e.options[e.selectedIndex].text;
                    
                   
                    
                    document.paymentDetails.categoryId.value =e.options[e.selectedIndex].value;
                    var categoryId1='C592823';
                    
                    var catsel='GUEST FEE';
                    
                    
                    
                    var selCategory=document.paymentDetails.categoryId.value;
                    
                    
                    if(selCategory == catsel)
                    	{
                    	 document.paymentDetails.categoryId.value=categoryId1;
                    	}
                    
                  
         				var dynaDobName=document.getElementById(&quot;dobName&quot;).value;  //EMRO
         				
         				
         				if( document.getElementById(&quot;formInd&quot;).checked)
         					{
         					
         					document.getElementById(&quot;individual&quot;).value=&quot;selected&quot;;
         					document.getElementById(&quot;organisation&quot;).value=null;
         					
         				 if(document.getElementById(&quot;cusName&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName&quot;).value == '')
	                	  {
	                	 alert(&quot;Name should not be empty &quot;);
	                	 return false;
	                	  }
         				 
 				 
	                	  var myVar=document.getElementById(&quot;cusName&quot;).value;
	                	  
	                	  var hasNumber = /\d/;  
	                	 
	                	 if(hasNumber.test(myVar))
	                		 {
	                		 alert(&quot;Name should be alphabet&quot;);
	                		 return false;
	                		 
	                		 }
	                  
	                 if(document.getElementById(&quot;dateOfBirth&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth&quot;).value == '')
	            	 {
	            	 alert(&quot;datofbirth should not be empty &quot;);
	            	 return false;
	            	  }
	                
	                
	                
	                  if(document.getElementById(&quot;mobileNo&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo&quot;).value == '')
	            	  {
	            	    alert(&quot;mobileNo should not be empty &quot;);
	            	    return false;
	            	  }
	                  
	                  var mNumber=document.getElementById(&quot;mobileNo&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
	                
	                  
	                  
	                  /*if(document.getElementById(&quot;emailId&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId&quot;).value == '')
	            	  {
	            	    alert(&quot;emailId should not be empty &quot;);
	            	    return false;
	            	  }*/
	                  
	                  var x=document.getElementById(&quot;emailId&quot;).value;
	                  
	                  
	                 if(x!= '' &amp;&amp; x!=null){ 
	                  /* var atposition=x.indexOf(&quot;@&quot;);  
	                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
	                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
	                    alert(&quot;Please enter valid email id&quot;);  
	                    return false;  
	                    } */
	                   x= x.trim();
	                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
		         			alert(&quot;Please enter valid email id&quot;);
		         			return false;
		         		}
	                 }
	                 		
	                  
	                  

	                  var number12= document.getElementById(&quot;mobileNo&quot;).value;
	                	       number12=number12.trim();
	                	                  if(number12 == &quot;&quot;) {
	                	            	        window.alert(&quot;Mobile number should not be empty&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }

	                	            	    if(number12.length != 10) {
	                	            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }
	                  
	                  
	                  
         				
         					}	
	                  
	                  
         				
         				
         				
         				if( document.getElementById(&quot;fileOrg&quot;).checked )
     					{
         					
        					document.getElementById(&quot;individual&quot;).value=null;
         					document.getElementById(&quot;organisation&quot;).value=&quot;selected&quot;;	
     				
     				 if(document.getElementById(&quot;cusName12&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName12&quot;).value == '')
                	  {
                	 alert(&quot;Name should not be empty &quot;);
                	 return false;
                	  }
     				 
     				 
     				  var myVar=document.getElementById(&quot;cusName12&quot;).value;
                	  
                	  var hasNumber = /\d/;  
                	  
                	 if(hasNumber.test(myVar))
                		 {
                		 alert(&quot;Name should be alphabate&quot;);
                		 return false;
                		 
                		 }
     				 
     				 
                  
                  if(document.getElementById(&quot;dateOfBirth12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth12&quot;).value == '')
            	  {
            	 alert(&quot;datofbirth should not be empty &quot;);
            	 return false;
            	  }
                
                  if(document.getElementById(&quot;orgName&quot;).value == &quot;null&quot; || document.getElementById(&quot;orgName&quot;).value == '')
            	  {
            	 alert(&quot;orgName should not be empty &quot;);
            	 return false;
            	  }
                  var myVar1=document.getElementById(&quot;orgName&quot;).value;
            	  
            	  var hasNumber1 = /\d/;  
            	  
            	 if(hasNumber1.test(myVar1))
            		 {
            		 alert(&quot;Organization Name should be alphabate&quot;);
            		 return false;
            		 
            		 }
                  
                  if(document.getElementById(&quot;mobileNo12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo12&quot;).value == '')
            	  {
            	    alert(&quot;mobileNo should not be empty &quot;);
            	    return false;
            	  }
                  
               var number1= document.getElementById(&quot;mobileNo12&quot;).value;
                  
                  number1 = number1.trim();
                  if(number1 == &quot;&quot;) {
            	        window.alert(&quot;Mobile number should not be empty&quot;);
            	        number1.focus();
            	        return false;
            	    }

            	    if(number1.length != 10) {
            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
            	        number1.focus();
            	        return false;
            	    }
               
            	    
            	    var mNumber=document.getElementById(&quot;mobileNo12&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
            	    
            	    
            	  
                  /*if(document.getElementById(&quot;emailId12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId12&quot;).value == '')
            	  {
            	    alert(&quot;emailId should not be empty &quot;);
            	    return false;
            	  }*/
                  
                  
                  
                  
                  
                  
                 var x=document.getElementById(&quot;emailId12&quot;).value;
                 if(x != '' &amp;&amp; x!=null){ 
                  /* var atposition=x.indexOf(&quot;@&quot;);  
                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
                    alert(&quot;Please enter valid email id&quot;);  
                    return false;  
                    } */
                    x=x.trim();
                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
	         			alert(&quot;Please enter valid email id&quot;);
	         			return false;
	         		}
                 }
     					}	
         				
         				var corpdata=document.getElementById(&quot;corporate_1&quot;)
         				
         				var captchaOption = $(&quot;input[name='optionOfCaptcha']:checked&quot;).val();
        				document.getElementById('capchaRadioSel').value=captchaOption;
         	    		//alert(&quot;captchaOption :: &quot;+captchaOption);
         				
         				var reqDataTmp = 'cusName~dateOfBirth~mobileNo~captchaValue~cusName12~dateOfBirth12~mobileNo12~captchaValue12~capchaRadioSel';//img Aud Captcha
         				//alert(&quot;b4 check empty :&quot;+reqDataTmp);
    					var reqData = checkEmpty(reqDataTmp)
    					//alert(&quot;reqData is &quot;+reqData);
    					var reqDataEmail=document.getElementById(&quot;emailId&quot;)!=null?document.getElementById(&quot;emailId&quot;).value:'';
    				    reqData+='emailId='+reqDataEmail+'&amp;';
    				   // alert(&quot;reqDataEmail is &quot;+reqDataEmail);
    				    var reqDataEmail12=document.getElementById(&quot;emailId12&quot;)!=null?document.getElementById(&quot;emailId12&quot;).value:'';
    				    reqData+='emailId12='+reqDataEmail12+'&amp;';
    				   //alert(&quot;reqDataEmail12 is &quot;+reqData);
    				    var encdata = getEncryptData(reqData)
    					$(&quot;#encStaticData&quot;).val(encdata);
    				    //alert(&quot;encStaticData is &quot;+encStaticData);

    				 var len = document.getElementById(&quot;paramLen&quot;).value;
    				    
    				  for(i=1,k=11;i&lt;=len;i++,k++)
                     	{ 
                	 var temp;
                	
					if(document.getElementsByName(&quot;outref&quot;+k).length >0){
						
					document.getElementsByName(&quot;outref&quot;+k)[0].value='';
					
                	 } 

                }	
               
    				    
    				  /*   var revampcatId=document.getElementById(&quot;categoryId&quot;)!=null?document.getElementById(&quot;categoryId&quot;).value:'';  
         				var revampInstId=document.getElementById(&quot;instituteId&quot;)!=null?document.getElementById(&quot;instituteId&quot;).value:'';  
         				var revampInstutionId=document.getElementById(&quot;institutionId&quot;)!=null?document.getElementById(&quot;institutionId&quot;).value:'';
         				var revampState=document.getElementById(&quot;state&quot;)!=null?document.getElementById(&quot;state&quot;).value:'';
         				var revampCategoryName=document.getElementById(&quot;categoryName&quot;)!=null?document.getElementById(&quot;categoryName&quot;).value:'';
         				var revampTransactioName=document.getElementById(&quot;transactionName&quot;)!=null?document.getElementById(&quot;transactionName&quot;).value:'';
         				var revampcredit=document.getElementById(&quot;creditTxnCount&quot;)!=null?document.getElementById(&quot;creditTxnCount&quot;).value:'';
         				var revampDebit=document.getElementById(&quot;debitBranchCode&quot;)!=null?document.getElementById(&quot;debitBranchCode&quot;).value:'';
         				var revampDob=document.getElementById(&quot;dobName&quot;)!=null?document.getElementById(&quot;dobName&quot;).value:'';
         				var revampCreditBranch=document.getElementById(&quot;creditBranchCode&quot;)!=null?document.getElementById(&quot;creditBranchCode&quot;).value:'';
         				var revampCorp=document.getElementById(&quot;corpName&quot;)!=null?document.getElementById(&quot;corpName&quot;).value:'';
         				var revampCorpAdd=document.getElementById(&quot;corpAddress&quot;)!=null?document.getElementById(&quot;corpAddress&quot;).value:'';
         				var revampInst=document.getElementById(&quot;instType&quot;)!=null?document.getElementById(&quot;instType&quot;).value:'';
         				var revampLogopath=document.getElementById(&quot;logoPath&quot;)!=null?document.getElementById(&quot;logoPath&quot;).value:'';
         				var revampLogoName=document.getElementById(&quot;logoName&quot;)!=null?document.getElementById(&quot;logoName&quot;).value:'';
         				var revampIndividual=document.getElementById(&quot;individual&quot;)!=null?document.getElementById(&quot;individual&quot;).value:'';
         				var revampCategory=document.getElementById(&quot;Category&quot;)!=null?document.getElementById(&quot;Category&quot;).value:'';
         				var revampValid=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:'';
         				var revampTrans=document.getElementById(&quot;transactionRemarks&quot;)!=null?document.getElementById(&quot;transactionRemarks&quot;).value:'';
         				var revampSelected=document.getElementById(&quot;selectAccountFrom&quot;)!=null?document.getElementById(&quot;selectAccountFrom&quot;).value:'';
         				var revampCorporateNet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:'';
         				var revampValidNotMand=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:'';
         				var revampValidMand=document.getElementById(&quot;validateMandatory&quot;)!=null?document.getElementById(&quot;validateMandatory&quot;).value:'';
         				var revampCorporateInternet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:'';
         				 */
         				
         				 
         				 
         				  //CR-9184 installment changes start
      	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='blank')
      	            	{
      	                 	alert(&quot;please select payment method&quot;);
      		                return false;
      	                	}
      	                //CR-9184 installment changes end
      					 var val = document.getElementById(&quot;transactionRemarks&quot;).value;
         				 //alert(&quot;Remarks :: &quot;+val);alert(&quot;Remark boolean :: &quot;+val.match(/^[A-Za-z0-9 .,-_/()#\t\r\n\f]+$/i.test($.trim(val)));
         				//alert(&quot;Remark boolean :: &quot;+val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f]*$/)); //^[A-Za-z0-9@_/,&lt;>:^;()# .-]*$
         				if(val != '' &amp;&amp; !(val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f ]*$/)) &amp;&amp; val != null &amp;&amp; val.trim()){
         					alert(&quot;Please enter Remarks in valid format!&quot;);
         					return false;
         				}
         				
 						var encmis = 'categoryId~instituteId~institutionId~state~categoryName~dobName~corpName~corpAddress~instType~logoPath~logoName~organisation~individual~Category~transactionRemarks~CorporateInternetBanking~selectAccountFrom~validateMandatory~organisation~formInd~fileOrg';
         				var reqData = checkEmpty(encmis)
         				var encdata = getEncryptData(reqData)
     					$(&quot;#encMis&quot;).val(encdata);
     				   
     				    
     				   /*  if(document.getElementById(&quot;instType&quot;)!=null)
     				    	{
         				    alert(&quot;before::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	document.getElementById(&quot;instType&quot;).value='';
         				    alert(&quot;After::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	} */
     				    
     				   /*  if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
     				    	{
     				    	document.getElementById(&quot;validateNotMandatory&quot;)=null;

     				    	//document.getElementById(&quot;validateNotMandatory&quot;).value='';
     				    	}
 */     				    
     				  /*  if(document.getElementById(&quot;validateMandatory&quot;)!=null)
				    	{
				    	document.getElementById(&quot;validateMandatory&quot;).value='';
				    	} */
     				    
     				  /* if(document.getElementById(&quot;CorporateInternetBanking&quot;)!=null)
				    	{
				    	document.getElementById(&quot;CorporateInternetBanking&quot;).value='';
				    	}  
				    	
				  	  if(document.getElementById(&quot;selectAccountFrom&quot;)!=null)
				    	{
				    	document.getElementById(&quot;selectAccountFrom&quot;).value='';
				    	}  */

     				//alert(&quot;Hi &quot;+document.getElementById(&quot;validateNotMandatory&quot;).value);alert(&quot;2 &quot;+document.getElementById(&quot;organisation&quot;).value);
     				
     				   //removed from here
   		         
   		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
    				    
    				    
    				    
    				    
         				
         				if(!corpdata.checked)
         					{
         					alert(&quot;Please read and accept the Terms and Conditions&quot;);
         					return false;
         					}
    				
         				var capt=document.getElementById(&quot;captchaValue&quot;).value
         				
         				if(capt == &quot;null&quot; || capt == '')
         					{
         					
         					alert(&quot;Please enter captcha value as shown in the image or from the audio&quot;);
         					return false;
         					}
         				else
         					{
         				//	alert(&quot;Am here!!&quot;);
         				//	alert(capt);
         					//	
         					var alphaNumExp = new RegExp(&quot;^[0-9a-zA-Z]+$&quot;);
         					 if(!document.getElementById(&quot;captchaValue&quot;).value.match(alphaNumExp)) {
								alert('Please enter valid text as shown in the image or from the audio');
								document.getElementById(&quot;captchaValue&quot;).value=&quot;&quot;;
								document.getElementById(&quot;captchaValue&quot;).focus();
								return false;
						      }else{
         							validateCaptcha('paymentDetails',capt);
         					  }
         					}
         				
     	                
         				// $('#instType').val('');   
       				    // $('#instType').removeAttr('name');
         				  
         				  
         				 //$('#validateNotMandatory').val('');
      	              //   $('#validateMandatory').val('');  
      	               
         				  
         				  //$('#validateNotMandatory').removeAttr('name');
      		             // $('#validateMandatory').removeAttr('name');
      		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
      		             //alert(&quot;data is &quot;+document.getElementById(&quot;validateMandatory&quot;));
      		             
      		             
      		             	//CR-9184 installment changes start
					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='partial')
					{
					   //alert(&quot;one&quot;);
						$('#paymentDetails').attr('action', 'installmentdetails.htm');
						}
					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='full'))
						{
						//alert(&quot;two&quot;);
						$('#paymentDetails').attr('action', 'installmentdetailsconfirmpage.htm');
						}
					//CR-9184 installment changes end
					else
					   {
						//alert(&quot;three&quot;);
					$('#paymentDetails').attr('action', 'confirmpayment.htm');
					 }
				
       				//$('#paymentDetails').attr('action', 'confirmpayment.htm');
        				
         				
         }
 
         	function validateCaptcha(formName, captchaValue) {
         		//img Aud Captcha
        		var captchaOption = $(&quot;input[name='optionOfCaptcha']:checked&quot;).val();
        		document.getElementById('capchaRadioSel').value=captchaOption;
         		
         	      var captcha = $('#' + captchaValue).val();
         	      
         	     captcha=captchaValue;
         	      
         	      var validateurl = &quot;/sbicollect&quot;+'/validatecaptcha.htm';
         	     
         	    var reloadurl = &quot;/sbicollect&quot;+'/simpleCaptchaServ?'; //img Aud Captcha 
         	      var errorurl = &quot;/sbicollect&quot;+'/sessiontimeout.htm' ; 
         	    
         	       var successurl = &quot;/sbicollect&quot;+'/payment/confirmpayment.htm' ;
         	      
         	       var installmentDetails = &quot;/sbicollect&quot;+'/payment/installmentdetails.htm' ;
         	     
         	       var installmentDetailsConfirm = &quot;/sbicollect&quot;+'/payment/installmentdetailsconfirmpage.htm' ;
         	    //audio
         			if(captchaOption =='AUD')
         				reloadurl = &quot;/sbicollect&quot;+'/audio.wav?bogus=';
         	      
         	      $.ajax({
         	            type : 'POST',
         	            dataType : 'text',
         	            url : validateurl,
         	          //  data : 'captchaVal=' + captcha,//commented for img Aud Captcha
		    				data:jQuery.param({captchaVal:captcha,captchaOption:captchaOption}),
         	            success : function(data) { 
         	            	
         	            	
         	            	
         	                  if (data == 'success') {
         	                	  
         	                	 $('#transactionName').val('');$('#creditTxnCount').val('');$('#debitBranchCode').val('');$('#creditBranchCode').val('');$('#installmentParamKey').val('');$('#categoryId').val('');$('#categoryName').val('');$('#netbankingflag').val('');$('#branchflag').val('');$('#creditcardflag').val('');
         	              		 $('#otherdebitflag').val(''); $('#installmentParamKey').val('');
         	              		
         	              		 $('#controls').val('');
         	                	  
         	                	 $('#paramLen').val('');   
        				    	 $('#validateNotMandatory').val('');
        				    	 $('#validateMandatory').val('');
             	               $('#instType').val('');
             	                //$('#validateNotMandatory').val('');
             	               // $('#validateMandatory').val('');  
             	                //$('#selectAccountFrom').val('');
             	                $('#CorporateInternetBanking').val('');  
             	               $('#organisation').removeAttr('name');
             				   $('#categoryId').removeAttr('name');
           		               $('#instituteId').removeAttr('name');
           		               $('#institutionId').removeAttr('name');
           		               $('#state').removeAttr('name');
           		               $('#categoryName').removeAttr('name');
           		               //$('#transactionName').removeAttr('name');
        		               //$('#creditTxnCount').removeAttr('name');
        		               //$('#debitBranchCode').removeAttr('name');
        		               $('#dobName').removeAttr('name');
        		               //$('#creditBranchCode').removeAttr('name');
        			
        		               //$('#corpName').removeAttr('name');
           		               $('#corpAddress').removeAttr('name');
           		               //$('#instType').removeAttr('name');
           		               $('#logoPath').removeAttr('name');
           		               $('#logoName').removeAttr('name');
          			
           		               $('#individual').removeAttr('name');
        		               $('#Category').removeAttr('name');
        		              // $('#validateNotMandatory').removeAttr('name');
        		               $('#transactionRemarks').removeAttr('name');
        		              // $('#selectAccountFrom').removeAttr('name');
        			
        		               //$('#CorporateInternetBanking').removeAttr('name');
           		              // $('#validateNotMandatory').removeAttr('name');
           		               //$('#validateMandatory').removeAttr('name');
           		              // $('#CorporateInternetBanking').removeAttr('name');
           		               $('#cusName').removeAttr('name');
          			
           		               $('#dateOfBirth').removeAttr('name');
        		               $('#mobileNo').removeAttr('name');
        		               $('#captchaValue').removeAttr('name');
        		               $('#cusName12').removeAttr('name');
        		               $('#dateOfBirth12').removeAttr('name');
        		               $('#capchaRadioSel').removeAttr('name');//imag and audio captcha
        		               $('#mobileNo12').removeAttr('name');
           		               $('#captchaValue12').removeAttr('name');
           		               $('#emailId').removeAttr('name');
           		               $('#emailId12').removeAttr('name');
           		               $('#formInd').removeAttr('name');
           		               $('#fileOrg').removeAttr('name');  
           		               $('#corporate_1').removeAttr('name');
         	                	  
         	                 	//CR-9184 installment changes start
         	 					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='partial')
         	 					{
         	 						$('form#paymentDetails').attr({
      	                              action : installmentDetails
      	                        	});
         	 						}
         	 					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value=='full'))
         	 						{
         	 						 $('form#paymentDetails').attr({
        	                              action : installmentDetailsConfirm
        	                        });
         	 						 }
         	 					//CR-9184 installment changes end
         	 					else
         	 					   {
         	 						 $('form#paymentDetails').attr({
        	                              action : successurl
        	                        }); 
         	 						 
         	 					   }
         	 				
         	                 	
         	                    //   alert(&quot;data in captcha&quot;+document.getElementById(&quot;validateMandatory&quot;));

         	                    //  document.getElementById(&quot;validateMandatory&quot;)=null;
         	                    //alert(&quot;1111111111111111111111111111111111111111&quot;);
         	                       $('form#paymentDetails').submit();
         	                      
         	                  }else if(data =='sessiontimeout'){
         	                        $('form#paymentDetails').attr({
         	                              action : errorurl
         	                        });   
         	                                                
         	                        $('form#paymentDetails').submit();
         	                  }else {

         	                        alert('Please enter valid captcha as shown in the image or from the audio');
         	                      //  $('#captcha').val(&quot;&quot;);
         	                      $('#captchaValue').val(&quot;&quot;);
         	                       document.getElementById(&quot;captchaValue&quot;).value = &quot;&quot;;
         	                        document.getElementById(&quot;imageContainer&quot;).removeChild(
         	                    //    document.getElementById(&quot;captchaImage&quot;));
         	                        		document.getElementById(&quot;refreshImgCaptcha&quot;));//img Aud Captcha
         	                        var newImg = document.createElement('img');
         	                      //  newImg.setAttribute('id', &quot;captchaImage&quot;);
         	                      newImg.setAttribute('id', &quot;refreshImgCaptcha&quot;);//img Aud Captch
         	                        newImg.setAttribute('src', reloadurl+ new Date().getTime());
         	                        newImg.setAttribute('alt', &quot;Captcha&quot;);
         	                        var div = document.getElementById('imageContainer');
         	                        div.appendChild(newImg);

         	                  }
         	            },
         	            error : function(data) {
         	                  alert('error : ' + data);
         	            }
         	     
         	      }); 
         	    // document.getElementById(&quot;validateMandatory&quot;)=null;
         	      
         	      
         	}  

     
         	function captchaRefresh()
         	{
         		$('#captcha').val(&quot;&quot;);
                 document.getElementById(&quot;imageContainer&quot;).removeChild(
                             document.getElementById(&quot;captchaImage&quot;)); 
                             var reloadurl = &quot;/sbicollect&quot;+'/loadcaptcha.htm?';
                 var newImg = document.createElement('img');
                 newImg.setAttribute('id', &quot;captchaImage&quot;);
                 newImg.setAttribute('src', reloadurl+ new Date().getTime());
                 newImg.setAttribute('alt', &quot;Captcha&quot;);
                 var div = document.getElementById('imageContainer');
                 div.appendChild(newImg);
         	}
         	
         	
         	
         	
         	function backInstitution()	{
         		document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		document.paymentDetails.submit();
         	}

         	function resetFeeParams(formName)	{

         		if(document.getElementById('validateMandatory') != null){
         			var validateMandatory=eval(formName+'.validateMandatory');		
         			if(validateMandatory.length==undefined)
         				validateMandatory = new Array(eval(formName+'.validateMandatory'));
         				
         			for(i=0;i&lt;validateMandatory.length;i++){
         				
         				var validations=validateMandatory[i].value.split('#SEP#');
         				var controlObj = eval(formName+'.'+validations[0]);
         				
         				if(controlObj.type.indexOf('select') != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		  }

         		if(document.getElementById('validateNotMandatory') != null)	{
         			
         			var validateNotMandatory=eval(formName+'.validateNotMandatory');		
         			if(validateNotMandatory.length==undefined)
         				validateNotMandatory = new Array(eval(formName+'.validateNotMandatory'));
         				
         			for(i=0;i&lt;validateNotMandatory.length;i++){
         				
         				var validationsNotMand =validateNotMandatory[i].value.split('#SEP#');
         				var controlObj = eval(formName+'.'+validationsNotMand[0]);
         				
         				if(controlObj.type.indexOf('select') != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		} 
         		
         		$('#cusName').val('');$('#dateOfBirth').val('');$('#mobileNo').val('');$('#emailId').val('');$('#captchaValue').val('');$('#transactionRemarks').val('');
         		
         	}
         	
         	
         	function OpenExcel(path,name)
         	{
         	   //var wind=window.open(path+name,&quot;Report&quot;,&quot;toolbar=yes,scrollbars=yes,resizable=yes,top=0,left=0,menuBar=yes&quot;);
         		path=path+name;//alert(&quot;full : &quot;+path);
        		var link=document.createElement('a');
        		link.href = path;
        		link.download = path.substr(path.lastIndexOf('/') + 1);
        		link.click();
         	}

         	
         	function getEncryptData(reqData)
         	{
         		var key = $(&quot;#eKeVal&quot;).val();
         		if(key != '')
         		{
         			
         			return CryptoJS.AES.encrypt(reqData,key)
         		}
         	}

         	
         	function checkEmpty(reqData)
         	{
         		var validInput = '';
         		if(reqData.indexOf(&quot;~&quot;) > 0)
         		{
         			var arr = reqData.split('~');
         			for(var i=0;i&lt;arr.length;i++)
         			{ 
         				var temp;
         				if(document.getElementById(arr[i]) !=null)
         				 temp = document.getElementById(arr[i]).value;
         				if(temp == &quot;&quot;) {
         					validInput += arr[i]+&quot;=&quot;+''+&quot;&amp;&quot;;
         				}else {
         					validInput += arr[i]+&quot;=&quot;+temp+&quot;&amp;&quot;;
         				}
         			}
         			return validInput;
         		}
         	
         	
 
         
         
         	}  
         	
         	
         	function myFunction1()
         	{
         		
     			 var w = document.paymentDetails.Category.selectedIndex;
    			 var categoryId1='C592823';
    			 var ins1='IN502269';
    			 var st1='Tamil Nadu';
    	
    			 
    			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
    		    document.getElementById(&quot;institutionId&quot;).value=ins1;
    			document.getElementById(&quot;state&quot;).value=st1;
    	
    	
    	var selected_text = document.paymentDetails.Category.options[w].text;
    	
    	if (selected_text == &quot;--Select Category--&quot;) {
    		 
    		
    		 return false;
    		 
    	} else {
    		
    		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
    		
    		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
    	
    		
    		var fileMode=category.options[category.selectedIndex].getAttribute('fileMode');
    	
    		var paramName=category.options[category.selectedIndex].getAttribute('paramName');
    		
    		var optStr=category.options[category.selectedIndex].getAttribute('optString');
    	
    		var dupPayment = category.options[category.selectedIndex].getAttribute('dupPayment');
    		
             //API    
    		if(fileMode=='Y' || fileMode=='A')
    		{
    			
    		
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
    				var arr = paramName.split(&quot;-&quot;);
    				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
    				
    				
    				
    				
    				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
    				
    				var arr = paramName.split(&quot;-&quot;);
        			
        			         				
    				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
    				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
    				
    				
    				
    				var url = &quot;/sbicollect&quot;+'/payment/filemode.htm';
    				$(&quot;#paymentDetails&quot;).attr('action', url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				

    			
    			
    			}
    		else
    			{
    			
    				
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;  
    				
    				
    				var url = &quot;/sbicollect&quot;+'/payment/listcategory.htm';
    				
    				
    				$(&quot;#paymentDetails&quot;).attr('action', url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				
    				return false; 
    			}
    		return true;
    		} 
    }
         	
         	
         	function restFeeParams(formName)	{
         		
         		var indOrOrgVal = &quot;&quot;; //def7 
         		if( $('#fileOrg').is(':checked') ){ //def7 
         			indOrOrgVal = &quot;fileOrg&quot;;
         		} 
         		
         	
         			 var selectedCaptcha = $('input[name=optionOfCaptcha]:checked').val();  //def 9
         	
         			
         		
          		 document.getElementById('loginAudioCaptcha').controls = false; //def 8
         		 document.getElementById('paymentDetails').reset();
         		document.getElementById('loginAudioCaptcha').controls = true; //def 8
         		// getUserSelImgCaptcha();
         		 
         		 if(indOrOrgVal == 'fileOrg'){ //def7 
         			$('#fileOrg').prop( &quot;checked&quot;, true );
         		 }else{
         			$('#formInd').prop( &quot;checked&quot;, true );
         		 }//def7 
         		 
         		
         		 if(selectedCaptcha == 'AUD'){  //def 9
         			$('input[name=optionOfCaptcha][value=AUD]').attr('checked', true);
         			getUserSelAudCaptcha();
         		 }else{
          			$('input[name=optionOfCaptcha][value=IMG]').attr('checked', true);
         			getUserSelImgCaptcha();
            		 
         		 } //def 9
         		}
         	function backFeeParams(formName)	{
         		
         		 
         		var insName='Educational';
         		//var insID ='IN502269'
         		document.getElementById('instType').value=insName;
         		//document.getElementById('institutionId').value=insID;
         		//document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		//document.paymentDetails.submit();
         		//Added by supriya for back button issue
         		var url = &quot;/sbicollect&quot;+'/payment/listinstitution.htm';
				
				
				$(&quot;#paymentDetails&quot;).attr('action', url);
				$(&quot;#paymentDetails&quot;).submit();
				//Added by supriya for back button issue
         	}
         	
         	 //Function to set the image captcha as default option on selection of individula/org radio button
         	function resetToImageCaptcha(){
         		if($('input:radio[name=optionOfCaptcha]:checked').val()=='AUD')
         	   {
         	   	$('input:radio[name=optionOfCaptcha]')[0].checked = true; 
               	getUserSelImgCaptcha();
         	   }
         	}
         	
         
	  




		
		
		
			
		
		
		 
        
            
                 © State Bank of India 
            
            
                Privacy Statement 
                 
                Disclosures
				 
				Terms of Use
            
        
		  		
		 
   
  
   $(document).ready(function() {
   var filename = window.location.href.substr(window.location.href.lastIndexOf(&quot;/&quot;)+1);
   
     switch(filename) {
	  case &quot;icollecthome.htm&quot;:   
	 $(&quot;#Link_1&quot;).addClass('active');
     break;
    case &quot;paymenthistory.htm&quot;:   
	 $(&quot;#Link_2&quot;).addClass('active');
     break;
    case &quot;faq.htm&quot;:    
    $(&quot;#Link_3&quot;).addClass('active');
     break;
	 case &quot;customersupport.htm&quot;:    
   $(&quot;#Link_4&quot;).addClass('active');
    break;
	 case &quot;paymenthistorydatedetails.htm&quot;:
	$(&quot;#Link_2&quot;).addClass('active');
	break;
	 case &quot;sendotp.htm&quot;:
	$(&quot;#Link_2&quot;).addClass('active');
	break;
  default:
   $(&quot;#Link_1&quot;).addClass('active');    
  } 
   
});


   
		
		
		
	
	


 
	


	




   	
 
	
  
      
       




			
				
					
					
					
					
					Thank you for choosing SB Collect. As per RBI guidelines on cross-border payment transactions, maximum amount of Rs 25,00,000/- per transaction / per item can only be processed on this platform. Hence, this transaction is declined. On clicking &quot;OK&quot; you will be redirected to SB Collect Corporate home page. Inconvenience regretted.
					
					
					
					OK
					
				
			



function logout()
{
	$('#logoutform').attr('action', 'logout.htm');
	$('#logoutform').submit();
}

function callURL(url)
{
	$('#logoutform').attr('action', url);
	$('#logoutform').submit();
}
function callLogout(url)
{
	$('#logoutform').attr('action', url);
	$('#logoutform').submit();
}

/* function callMopsPage() {
	
	var domain=document.domain;
	var win = window.open('https://'+domain+'/prelogin/mopsremittanceform.htm', '_blank');

//	var win = window.open('https://www.onlinesbi.com/prelogin/mopsremittanceform.htm', '_blank');

	if (win) {
	    win.focus();
	}
} */
function callInst(url)
{
	  // alert('callInst'+url);
	$('#logoutform').attr('action', url);
	$('#logoutform').submit();
}
function callCat(url){
//	alert('callact');
	$('#institutionform').attr('action', url);
	$('#institutionform').submit();
	
}
 function paySubmit(payflag)
	{
		//alert('payflag'+payflag);
		var hourCheck = $('#hourCheck').val();
		var minCheck = $('#minCheck').val();
		
		var foreignCardTxnLimit = parseFloat('');
		var amountTransfer      = parseFloat('');
		
		if(((hourCheck==23 &amp;&amp; minCheck>=30) || (hourCheck==00 &amp;&amp; minCheck&lt;=30) ) &amp;&amp; (payflag=='SBDEBIT' ||payflag=='OTHERDCARD'||payflag=='CREDITCARD' ||payflag=='PREPAID' ||payflag=='FOREIGN' ||payflag=='RUPAYCARD'))
		{
		 	alert(&quot;This payment mode is not available between 23:30 hours IST and 00:30 hours IST&quot;);
		}else if(((hourCheck == 22 &amp;&amp; minCheck >= 30) || (hourCheck == 23 &amp;&amp; minCheck &lt;= 30) ) &amp;&amp; (payflag == 'UPI'))
		{
		 	alert(&quot;This payment mode is not available between 22:30 hours IST and 23:30 hours IST&quot;);
		} else {
			
			$('#payflag').val(payflag);

			if( payflag=='SBDEBIT' ||payflag=='OTHERDCARD'||payflag=='CREDITCARD' ||payflag=='PREPAID' ||payflag=='FOREIGN' ||payflag=='RUPAYCARD') {
				$('#frmFeeParams').attr('action', 'fsspaymentgateway.htm');
			}else if(payflag == 'UPI'){
			$('#frmFeeParams').attr('action', 'upipayment.htm');
			
		
			}else {
				$('#frmFeeParams').attr('action', 'suvidhapayment.htm');
			}
			
			if(payflag=='FOREIGN' &amp;&amp; amountTransfer > foreignCardTxnLimit){
				$('#transactionModal').modal('show');
			}else{
				$('#frmFeeParams').submit();
			}		
		}
	}
 function submitVPA()
 { //alert('submitVPA');
 	/* var validator = $(&quot;#frmupi&quot;).validate (
 	{
 		focusCleanup: true,
 		onkeyup: false,
 		onblur: false,
 		onfocusout:false,
 		rules:{
 			vpa:{ required : true,
 				vpacheck : true}
 		},
 		messages: {
 			vpa :{
 				required :&quot;Please enter VPA&quot;,
 				vpacheck:&quot;You have entered an invalid VPA!&quot;
 				}
 		}
 	}); */
 	
 /* 	if (validator.form()) { */
 	//	doencrypt();
 		$('form#frmupi').attr({
 			action : 'validateVPA.htm'
 		});	
 		$('#frmupi').submit();
 	//}
 }




$(document).ready(function () {
	
	//Disable full page
/*   $(&quot;body&quot;).on(&quot;contextmenu&quot;,function(e) {
      return false;
  }); */
  
  $(this).bind(&quot;contextmenu&quot;, function(e) {
      e.preventDefault();
  });
	
  function disableBack() { window.history.forward() }
  
  window.onload = disableBack();
  window.onpageshow = function(evt) { if (evt.persisted) disableBack() }

    
  $(document).keydown(function (e) {
        return (e.which || e.keyCode) != 116;
  });
    
});




    $('.select2').select2();



/html[1]/body[1]/section[@class=&quot;section_div&quot;]/div[@class=&quot;col-lg-12  col-md-12 col-sm-12 col-12 content_section&quot;]</value>
      <webElementGuid>d5a6ad25-a13a-4607-b763-169b65f96841</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>66998b83-a180-4c4a-9017-62fd841948c8</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>8171f4e8-64b7-4678-9091-1fd8c51cc22b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
	
	
		

		
		
		
			




  

  
  
	 	
	
		  
	
	 	
	
		
	
	
				 
			 
				
					 SB Collect 
				
				
								
			 
			
			 
			

				
				
				
				
				  
					
					  Home 
					
					
					   Transaction
									History
							
					
					  FAQ&quot; , &quot;'&quot; , &quot;s
					
					
					  Customer Support
					
				  
				

			
			
			 
	
		
	
	
  
  

 

		
		
		
			












 


SB Collect












#loginAudioCaptcha{
	width:100% !important;
}
#refreshImgCaptcha
{
width:125px;
 margin-top: -8px;
}

@media only screen 
and (min-device-width : 768px) 
and (max-device-width : 1024px)  {
#refreshImgCaptcha
{
width:120px !important;
}
}
 
 
 
 $(document).ready(function() {
		getUserSelImgCaptcha();
		
});

	// Opera 8.0+
 var isOpera = (!!window.opr &amp;&amp; !!opr.addons) || !!window.opera || navigator.userAgent.indexOf(&quot; , &quot;'&quot; , &quot; OPR/&quot; , &quot;'&quot; , &quot;) >= 0;
 // Firefox 1.0+
	var isFirefox = typeof InstallTrigger !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;;
 // Safari 3.0+ &quot;[object HTMLElementConstructor]&quot; 
 var isSafari = /constructor/i.test(window.HTMLElement) || (function (p) { return p.toString() === &quot;[object SafariRemoteNotification]&quot;; })(!window[&quot; , &quot;'&quot; , &quot;safari&quot; , &quot;'&quot; , &quot;] || (typeof safari !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; safari.pushNotification));
 // Internet Explorer 6-11
 var isIE = /*@cc_on!@*/false || !!document.documentMode;
 // Edge 20+
 var isEdge = !isIE &amp;&amp; !!window.StyleMedia;
 // Chrome 1 - 71
 var isChrome = !!window.chrome &amp;&amp; (!!window.chrome.webstore || !!window.chrome.runtime);
 // Blink engine detection
 var isBlink = (isChrome || isOpera) &amp;&amp; !!window.CSS;
	//window.onload = function(){ 
	 
		//getUserSelImgCaptcha();
		
//	} 
  
  function getUserSelImgCaptcha() {
 	
 	if(null!=document.paymentDetails.captchaValue){//audio
 		document.paymentDetails.captchaValue.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; //audio
     	document.paymentDetails.capOption.value = &quot; , &quot;'&quot; , &quot;IMG&quot; , &quot;'&quot; , &quot;;
     //	document.getElementById(&quot; , &quot;'&quot; , &quot;noAudImgSelection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;audioselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;audelcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot;refreshImgCaptcha&quot;).src=&quot; , &quot;'&quot; , &quot;/sbicollect/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;+(new Date().getTime());

 	}
 	
	}


 function getUserSelAudCaptcha() {
 	document.paymentDetails.captchaValue.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
 	document.paymentDetails.capOption.value = &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;;
 	//document.getElementById(&quot; , &quot;'&quot; , &quot;noAudImgSelection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 	document.getElementById(&quot; , &quot;'&quot; , &quot;imgselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;audioselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;audelcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
		if (isIE == true){
			document.getElementById(&quot; , &quot;'&quot; , &quot;IEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot; , &quot;'&quot; , &quot;othrIEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		}else {
			document.getElementById(&quot; , &quot;'&quot; , &quot;othrIEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot; , &quot;'&quot; , &quot;IEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot;loginAudioCaptcha&quot;).src = &quot; , &quot;'&quot; , &quot;/sbicollect/audio.wav?bogus=&quot; , &quot;'&quot; , &quot; + new Date().getTime();
				
		}
	}
 
 function refreshImg()
 {
 	if(null!=document.getElementById(&quot;refreshImgCaptcha&quot;)){
 document.getElementById(&quot;refreshImgCaptcha&quot;).src=&quot; , &quot;'&quot; , &quot;/sbicollect/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;+(new Date().getTime());
 
 	}
 }   
 
 



	
		 
		 
			 
			 
			
		 
		
			 
			 
			 
			
		    
		    
		
		
		 
		 
		 
		 
		
		
		
		
 
		 
		

		
		 
		 
		
		

		
	    


		 
		  
		
		
		 
		
	
             
        
        
        
        
        
      






		
		 
		
			
				
				
					Payment Progress
					
						Select Payee
						Enter Payment Details
						Verify Payment Details
						Complete Payment
						Print Receipt
					
				
			
		
		
		
			
				 
				
				 BDU CHIEF HOSTEL ADMINISTRATOR  |  CENTRAL OFFICE FOR HOSTEL ADMINISTRATION, BHARATHIDASAN UNIVERSITY, , TRICHY-620024
				
	
			
			

			
				


					
						
							Enter Payment Details
							
								
									
									
									
									Payment Category*:
									
									
								
								
								
								
								
									 
									
									
										 --Select any Category-- 
										
											
												

													GUEST FEE
												
												
											
										
											
												
												

													
														HOSTEL ADMISSION FEE 
												
											
										
											
												
												

													
														HOSTEL MESS FEES 
												
											
										
									GUEST FEE
									
									
								
							
						
	

          
          
          	
			
									
								
			
			
				
				
				
				
				
				
				
		  	
			
						
					
						
					
						
						
							
							



NAME *










						
							
							



COURSE/PURPOSE *










						
							
							



DEPARTMENT *










						
							
							



NUMBER OF DAYS *










						
							
							



FROM (DD/MM/YYYY) *










						
							
							



TO (DD/MM/YYYY) *










						
							
							



MOBILE *










						
							
							



AMOUNT (In Rs,) *










						
						

						
							


								
									Remarks : 
								
								
									 
								
							
						
					
				
					
				
				
					Please confirm the detail before proceeding for payment. Save a copy of e-receipt for your future reference.
				
				
					For any queries please contact Office 0431-2407038
				
				
				


						
							Enter Your Details

							
								
									 Individual
								
								
									 Organisation
										/ Corporate
								
							
						



						
							
								
									
										Name * :
										
									
									
										
									
									
									
									
										
										Date of
											Birth * :
										
										
										 
										
									
										
									
									
										
											
											
										 
									
								
							
							
								
									
										Mobile No * :
										
									
									
										
										On successful completion of
											payment,you will receive the transaction reference number on
											this mobile number
									
									
									
										Email ID :
										
									
									
										
										On successful completion of payment,
											you will receive the transaction reference number on this
											email ID
									
								
							

						

						

							

								
									Your Name * :
									
								
								
									
								

								


								
									Organisation
										Name * :
									
								
								
									
								


							
							

								
								
									
									Date of
										Incorporation * :
									
									
									
									
								
										
								
								

									
										 
									
								

								


								
									Mobile No * :
									
								
								
									
									On successful completion of payment,you
										will receive the transaction reference number on this mobile
										number
								


							
							

								
									Email Id :
									
								
								
									
									On successful completion of payment,
										you will receive the transaction reference number on this
										email ID

								

								

							

						
				

						
							
								
									
										 I
											have read and agreed to the Terms
												&amp; Conditions

										
									
								
							
						
					


						
						
							
								
									Enter the text as shown in the
											image
										*:
									

								
								
									Enter the text as from audio *

								
								
								
									
										Select one of the Captcha options
											*
									
                                        
											Image
											Captcha
										
										 
											Audio
											Captcha
										
									

								
								
						
									 
											
											
												
											
								
								
									
											
											
										
										
										
										  
										
								
								
							
						
				
						
							
								
								
								Back
								
								
								
								
								Reset
								
								Next
								
								
							
						
					

				
			
			
			
		
		

		
			
				
					

					
					
						X
						Terms and Conditions
						  
						 
						
								Corporate Customer: Firm/Company/Institution (F/C/I) collecting payment from their beneficiaries.
								User: The beneficiary making a payment to F/C/I for the services/goods availed.
								Bank shall not be responsible, in any way, for the quality or merchantability of any product/merchandise or any of the services related thereto, whatsoever, offered to the User by the Corporate Customer. Any disputes regarding the same or delivery of the Service or otherwise will be settled between Corporate Customer and the User and Bank shall not be a party to any such dispute. Any request for refund by the User on any grounds whatsoever should be taken up directly with the Corporate Customer and the Bank will not be concerned with such a request.
								Bank takes no responsibility in respect of the services provided and User shall not be entitled to make any claim against the Bank for deficiency in the services provided by the Corporate Customer.
								The User shall not publish, display, upload or transmit any information prohibited under Rule 3(2) of the Information Technology (Intermediaries guidelines) Rules, 2011.
								In case of non-compliance of the terms and conditions of usage by the User, the Bank has the right to immediately terminate the access or usage rights of the User to the computer resource of the Bank and remove the non-compliant information.
							
					
					
					
						Ok
					
				
			
		

		
	








         $(document).ready(function(){
        	// $(function() {  
        	 //$.datepicker.setDefaults({
        		//   inline: true,
        		 //  showOn: &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;,
        		  // buttonImageOnly: true,
        		  // buttonImage: &quot; , &quot;'&quot; , &quot;/sbijava/sbcollectrevamp/images/calendar_icon.png&quot; , &quot;'&quot; , &quot;,
        		 //  buttonText: &quot; , &quot;'&quot; , &quot;Calendar&quot; , &quot;'&quot; , &quot;
        		  
        		//   }); 
        	// $(&quot;#dateOfBirth&quot;).datepicker({ 
        		// dateFormat: &quot;dd/mm/yy&quot;,
        		// yearRange: &quot;-150:+0&quot;,
        		    
        		// maxDate: new Date()
        		// }).val()          
        		//  var changeMonth = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot; ); 
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot;, true ); 
        		// var changeYear = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot; );
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot;, true );
        		// $( &quot;#dateOfBirth&quot; ).datepicker({ appendText: &quot;(dd/mm/yy)&quot; });
        	// });
        		 
        	 
	   $(&quot;#ForOrganisation&quot;).hide();
	   
        $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).click(function(){
           if($(this).attr(&quot;value&quot;)==&quot;formInd&quot;){
               $(&quot;#ForOrganisation&quot;).hide();    
               $(&quot;#ForIndividual&quot;).show();
              //Added as part of Reset defect
                    $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot;#dateOfBirth12&quot;).datepicker(&quot; , &quot;'&quot; , &quot;setDate&quot; , &quot;'&quot; , &quot;, null); 
                    $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();//def-6
                    resetToImageCaptcha();
                                   
                }
          else if ($(this).attr(&quot;value&quot;)==&quot;fileOrg&quot;){
               $(&quot;#ForIndividual&quot;).hide();
              $(&quot;#ForOrganisation&quot;).show();
			   //$(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                   $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                   $(&quot;#dateOfBirth&quot;).datepicker(&quot; , &quot;'&quot; , &quot;setDate&quot; , &quot;'&quot; , &quot;, null);
                   $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                   $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus(); 
                   $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus(); //def-6
                   resetToImageCaptcha();
      }
      });
	   //$(&quot;#exampleModalLong&quot;).modal(&quot;show&quot;);
	  });
        
// Somnath made change starts here
	//$(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).datepicker({
      //             format:&quot; , &quot;'&quot; , &quot;dd/mm/yyyy&quot; , &quot;'&quot; , &quot;,
        //      }).datepicker(&quot;setDate&quot;,&quot; , &quot;'&quot; , &quot;now&quot; , &quot;'&quot; , &quot;);
// Somnath made change end here
      //  $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).datepicker({
            //        format:&quot; , &quot;'&quot; , &quot;dd/mm/yyyy&quot; , &quot;'&quot; , &quot;,
             //   }).datepicker(&quot;setDate&quot;,&quot; , &quot;'&quot; , &quot;now&quot; , &quot;'&quot; , &quot;);
/*         
        $(function() {
        	 $(&quot;#dateOfBirth&quot;).datepicker({  maxDate: new Date() });
         });
         
         
         
         $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).datepicker({
            format: &quot;dd/mm/yyyy&quot;,
           	autoclose: true,
           	orientation: &quot;top&quot;,
          	endDate: &quot;today&quot;

       });
         
         
        $(&quot;.dateOfBirth&quot;).datepicker({
            maxDate: &quot;-1d&quot;, 
     		minDate: new Date(2007, 6, 12)
     });
         
*/         
//	Somnath made change starts here 
		[&quot;dateOfBirth&quot;,&quot;dateOfBirth12&quot;].forEach(function(val) { 
			$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+val).datepicker({autoclose:true});
		});
// 	Somnath made change starts ends 
         
         function selectCategory(category)
         {
           //alert(&quot;Entered into select category onchange&quot;);
         	var w = document.paymentDetails.Category.selectedIndex;
         	
        			  
         			  
         			 var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
         			 var ins1=&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;;
         			 var st1=&quot; , &quot;'&quot; , &quot;Tamil Nadu&quot; , &quot;'&quot; , &quot;;
         	
         			 
         			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
         			document.getElementById(&quot;institutionId&quot;).value=ins1;
         			document.getElementById(&quot;state&quot;).value=st1;
         	
         	
         	var selected_text = document.paymentDetails.Category.options[w].text;
         	
         	if (selected_text == &quot;--Select Category--&quot;) {
        		 return false;
         		 
         	} else {
         		
         		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
         		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
         		         		
         		var fileMode=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;fileMode&quot; , &quot;'&quot; , &quot;);
         		
         		var paramName=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;paramName&quot; , &quot;'&quot; , &quot;);
         		
         		var optStr=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;optString&quot; , &quot;'&quot; , &quot;);
         		
         		var dupPayment = category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;dupPayment&quot; , &quot;'&quot; , &quot;);
         		
                  //API    
         		if(fileMode==&quot; , &quot;'&quot; , &quot;Y&quot; , &quot;'&quot; , &quot; || fileMode==&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;)
         		{
         			
         			
         				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
         				var arr = paramName.split(&quot;-&quot;);
         				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
         				
       				
         				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
         				
         				var arr = paramName.split(&quot;-&quot;);
             			
             			         				
         				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
         				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
         				
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateNotMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				//document.getElementById(&quot;controls&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;formInd&quot;) !=null)
         				document.getElementById(&quot;formInd&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;fileOrg&quot;)!=null)
         				document.getElementById(&quot;fileOrg&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;corporate_1&quot;)!=null) 
         					document.getElementById(&quot;corporate_1&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				  
         				var onloadenc=&quot; , &quot;'&quot; , &quot;numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~firstkey~secondkey~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha&quot; , &quot;'&quot; , &quot;
         					var reqData = checkEmpty(onloadenc);
         					var encdynamicdata = getEncryptData(reqData);
        					$(&quot;#enconload&quot;).val(encdynamicdata);
         				
         				
         				
         				
         				
         				   $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				   $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
         				
         				   $(&quot; , &quot;'&quot; , &quot;#numberofkeyfield&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		              // $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
         				
      		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#fileMode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#encData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#encStaticData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#encdynamicData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#encKey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		              // $(&quot; , &quot;'&quot; , &quot;#encKeyVal&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
         				
    		               $(&quot; , &quot;'&quot; , &quot;#encMis&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref11&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref13&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#outref14&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               
    		               $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;); //audio image captcha
    		               $(&quot; , &quot;'&quot; , &quot;#Ur02H&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		              // $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
         				
         				
         				
    				    var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/filemode.htm&quot; , &quot;'&quot; , &quot;;
         				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
         				$(&quot;#paymentDetails&quot;).submit();
       					return false;
 
         			}
         		else
         			{
         			document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;
         			
         			var onloadencCatList=&quot; , &quot;'&quot; , &quot;numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~categoryNameSel~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha
     					var reqData = checkEmpty(onloadencCatList);
     					var encdynamicdata = getEncryptData(reqData);
    					  $(&quot;#encData&quot;).val(encdynamicdata);
     				
     				   $(&quot; , &quot;'&quot; , &quot;#numberofkeyfield&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		              $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
     				
  		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#fileMode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		             $(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               //$(&quot; , &quot;'&quot; , &quot;#encData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		             $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		               $(&quot; , &quot;'&quot; , &quot;#encStaticData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#encdynamicData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		              // $(&quot; , &quot;'&quot; , &quot;#encKey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		              // $(&quot; , &quot;'&quot; , &quot;#encKeyVal&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
     				
		               $(&quot; , &quot;'&quot; , &quot;#encMis&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref11&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		              $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref13&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#outref14&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               
		               $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#Ur02H&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);//img Aud Captcha
		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         			
         			
         			
         			
         			
         			
 				
         				  
      				
         				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listcategory.htm&quot; , &quot;'&quot; , &quot;;
    				    $(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
         				$(&quot;#paymentDetails&quot;).submit();
       				    return false; 
         			}
         		return true;
         		} 
         }

       
         
          function validateAndSubmitFeeParams(formName)
           {
       	  if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;) != null) {
        		
               //alert(&quot;inside validateManndatory data is &quot;+document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;));
        						var validateMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;);
        						
        						if(validateMandatory.length==undefined)
        							validateMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;));
        							
       							
        						for(i=0;i&lt;validateMandatory.length;i++) {
        							
        							var validations=validateMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         							var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validations[0]);

        							//Start of Code for Employee and Employer Contribution validation 
        							if(document.getElementById(&quot; , &quot;'&quot; , &quot;Category&quot; , &quot;'&quot; , &quot;))
        							{
        								 if(document.getElementById(&quot; , &quot;'&quot; , &quot;Category&quot; , &quot;'&quot; , &quot;).value ==&quot; , &quot;'&quot; , &quot;C005942&quot; , &quot;'&quot; , &quot;)	{
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Employee Contribution&quot; , &quot;'&quot; , &quot;){
        											employerContribution = controlObj.value;
        											employerId = validations[0];
        											}
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Employer Contribution&quot; , &quot;'&quot; , &quot;){
        											employeeContribution = controlObj.value
        											employeeId=validations[0];
        											}
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Number of Employees&quot; , &quot;'&quot; , &quot;) {
        											
        											noOfEmp=controlObj.value;
        											if(noOfEmp*3 != employerContribution){
        												alert(&quot;Enter correct value for Employee Contribution&quot;);
        												document.getElementById(employerId).value=&quot;&quot;;
        												return false;
        											}
        											
        											if(noOfEmp*6 != employeeContribution){
        												alert(&quot;Enter correct value for Employer Contribution&quot;)
        												document.getElementById(employeeId).value=&quot;&quot;;
        												return false;
        											}
        										}
        								
        								} 
        							}
         					//End of Code for Employee and Employer Contribution validation 
         					
         					if(controlObj.value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
         						if(validations[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         							alert(validations[3]);
         						else{
         					
         							if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1)
         								alert(&quot;Select &quot; + validations[1]);
         							else
         								alert(&quot;Enter &quot; + validations[1]);
         						}
         						controlObj.focus();
         						return false
         					}
         					
         				  //CR-9184 installment changes start
         	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;blank&quot; , &quot;'&quot; , &quot;)
         	            	{
         	                 	alert(&quot;please select payment method&quot;);
         		                return false;
         	                	}
         	                //CR-9184 installment changes end
         	
         					if (validations[2]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[2]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         					{

         					if(controlObj.value!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; controlObj.value.search(validations[2])!=0){
         						if(validations[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
         							if(validations[2] == &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,75}$)&quot; , &quot;'&quot; , &quot;)
         								alert(validations[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validations[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validations[3]); 
         							}
         						}
         						else{
         								alert(&quot;Enter &quot; + validations[1]);
         						}		
         						controlObj.focus();
         						return false
         					}
         					}
         					
         				}  // for loop complete
         				
         				
         			}	
         			
         			if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateNotMandatory&quot; , &quot;'&quot; , &quot;) != null){
         				var validateNotMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;);
         				
         				
         				if(validateNotMandatory.length==undefined)
         					validateNotMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;));
         					
         				for(i=0;i&lt;validateNotMandatory.length;i++){
         					
         					var validationsNotMand =validateNotMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         					var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validationsNotMand[0]);
                 			//23032023 date issue - regex changed to accept date and month in 01 - 09
         					if(controlObj.value ==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2] != &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2] != &quot; , &quot;'&quot; , &quot;(^(\\d{1,20})(\\.[0-9][0-9])?$)&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2]!=&quot; , &quot;'&quot; , &quot;^(0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](0[1-9]|[1-9]|1[012])[- /.](19|20)\\d\\d$&quot; , &quot;'&quot; , &quot; &amp;&amp;  validationsNotMand[2]!=&quot; , &quot;'&quot; , &quot;^(0[1-9]|[1-9]|1[012])[- /.](0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](19|20)\\d\\d$&quot; , &quot;'&quot; , &quot;){
         						alert(validationsNotMand[3]);
         						return false;
         					}
         					if(controlObj.value!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; controlObj.value.search(validationsNotMand[2])!=0){
         						if(validationsNotMand[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validationsNotMand[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         						{
         							if(validationsNotMand[2] == &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)&quot; , &quot;'&quot; , &quot;)
         								alert(validationsNotMand[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validationsNotMand[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validationsNotMand[3]); 
         							}
         							controlObj.focus();
         							return false;
         						}
         					}
         					
         					
         				}
         		  	}
         			
         			
         			
         			 var len = document.getElementById(&quot;paramLen&quot;).value;
                    
                     var reqdynamicData = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                 	var reqrefData=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                      for(i=1,k=11;i&lt;=len;i++,k++)
                     	{
                     
                     	 var temp;
                     	
  						if(document.getElementsByName(&quot;outref&quot;+k).length >0){ 
 						 temp = document.getElementsByName(&quot;outref&quot;+k)[0].value; 
 						reqrefData = &quot;outref&quot;+k+&quot;=&quot;+temp+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
 						
 						reqdynamicData+=reqrefData;
 					
                     	 } 

                     }
                    
                      
                      if(document.getElementById(&quot;installment&quot;)!=null)
               		{
               		var reqInstallment=document.getElementById(&quot;installment&quot;).value;

               		reqdynamicData+=&quot; , &quot;'&quot; , &quot;installment=&quot; , &quot;'&quot; , &quot;+reqInstallment+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
  					} 

                      
                      
                      var reqtransactionName=document.getElementById(&quot;transactionName&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;transactionName=&quot; , &quot;'&quot; , &quot;+reqtransactionName+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                        
                     
                      
                      var reqcreditTxnCount=document.getElementById(&quot;creditTxnCount&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditTxnCount=&quot; , &quot;'&quot; , &quot;+reqcreditTxnCount+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqdebitBranchCode=document.getElementById(&quot;debitBranchCode&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;debitBranchCode=&quot; , &quot;'&quot; , &quot;+reqdebitBranchCode+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                  
                      var reqcreditBranchCode=document.getElementById(&quot;creditBranchCode&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditBranchCode=&quot; , &quot;'&quot; , &quot;+reqcreditBranchCode+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      var reqinstallmentParamKey=document.getElementById(&quot;installmentParamKey&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;installmentParamKey=&quot; , &quot;'&quot; , &quot;+reqinstallmentParamKey+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      
                      
                                       
                      var reqcategoryId=document.getElementById(&quot;categoryId&quot;).value;
                     
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;categoryId=&quot; , &quot;'&quot; , &quot;+reqcategoryId+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  	 
                      
                      
                      var reqcategoryName=document.getElementById(&quot;categoryName&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;categoryName=&quot; , &quot;'&quot; , &quot;+reqcategoryName+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqnetbankingflag=document.getElementById(&quot;netbankingflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;netbankingflag=&quot; , &quot;'&quot; , &quot;+reqnetbankingflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      
                      var reqbranchflag=document.getElementById(&quot;branchflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;branchflag=&quot; , &quot;'&quot; , &quot;+reqbranchflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqcreditcardflag=document.getElementById(&quot;creditcardflag&quot;).value;
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditcardflag=&quot; , &quot;'&quot; , &quot;+reqcreditcardflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      var reqotherdebitflag=document.getElementById(&quot;otherdebitflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;otherdebitflag=&quot; , &quot;'&quot; , &quot;+reqotherdebitflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                    
                          					
                  	
                      var reqcontrols=document.getElementById(&quot;controls&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;controls=&quot; , &quot;'&quot; , &quot;+reqcontrols+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                     
                      //hidden fields ends
                      var reqparamLen = document.getElementById(&quot;paramLen&quot;).value;
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;paramLen=&quot; , &quot;'&quot; , &quot;+reqparamLen+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      //alert(&quot;reqdynamicData is &quot;+reqdynamicData);
                      //var reqData = checkEmpty(reqdynamicData)
                      var encdynamicdata = getEncryptData(reqdynamicData)
  					  $(&quot;#encdynamicData&quot;).val(encdynamicdata);
                      
                  	
              		 
                     	
                      
                      var e = document.getElementById(&quot;Category&quot;);
                      
                     
                      var strUser = e.options[e.selectedIndex].text;
                      document.getElementById(&quot;categoryName&quot;).value=strUser;
                      
                    
                     
                     var e = document.getElementById(&quot;Category&quot;);
                     var strUser = e.options[e.selectedIndex].text;
                    
                   
                    
                    document.paymentDetails.categoryId.value =e.options[e.selectedIndex].value;
                    var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
                    
                    var catsel=&quot; , &quot;'&quot; , &quot;GUEST FEE&quot; , &quot;'&quot; , &quot;;
                    
                    
                    
                    var selCategory=document.paymentDetails.categoryId.value;
                    
                    
                    if(selCategory == catsel)
                    	{
                    	 document.paymentDetails.categoryId.value=categoryId1;
                    	}
                    
                  
         				var dynaDobName=document.getElementById(&quot;dobName&quot;).value;  //EMRO
         				
         				
         				if( document.getElementById(&quot;formInd&quot;).checked)
         					{
         					
         					document.getElementById(&quot;individual&quot;).value=&quot;selected&quot;;
         					document.getElementById(&quot;organisation&quot;).value=null;
         					
         				 if(document.getElementById(&quot;cusName&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	                	  {
	                	 alert(&quot;Name should not be empty &quot;);
	                	 return false;
	                	  }
         				 
 				 
	                	  var myVar=document.getElementById(&quot;cusName&quot;).value;
	                	  
	                	  var hasNumber = /\d/;  
	                	 
	                	 if(hasNumber.test(myVar))
	                		 {
	                		 alert(&quot;Name should be alphabet&quot;);
	                		 return false;
	                		 
	                		 }
	                  
	                 if(document.getElementById(&quot;dateOfBirth&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	 {
	            	 alert(&quot;datofbirth should not be empty &quot;);
	            	 return false;
	            	  }
	                
	                
	                
	                  if(document.getElementById(&quot;mobileNo&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	  {
	            	    alert(&quot;mobileNo should not be empty &quot;);
	            	    return false;
	            	  }
	                  
	                  var mNumber=document.getElementById(&quot;mobileNo&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
	                
	                  
	                  
	                  /*if(document.getElementById(&quot;emailId&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	  {
	            	    alert(&quot;emailId should not be empty &quot;);
	            	    return false;
	            	  }*/
	                  
	                  var x=document.getElementById(&quot;emailId&quot;).value;
	                  
	                  
	                 if(x!= &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; x!=null){ 
	                  /* var atposition=x.indexOf(&quot;@&quot;);  
	                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
	                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
	                    alert(&quot;Please enter valid email id&quot;);  
	                    return false;  
	                    } */
	                   x= x.trim();
	                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
		         			alert(&quot;Please enter valid email id&quot;);
		         			return false;
		         		}
	                 }
	                 		
	                  
	                  

	                  var number12= document.getElementById(&quot;mobileNo&quot;).value;
	                	       number12=number12.trim();
	                	                  if(number12 == &quot;&quot;) {
	                	            	        window.alert(&quot;Mobile number should not be empty&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }

	                	            	    if(number12.length != 10) {
	                	            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }
	                  
	                  
	                  
         				
         					}	
	                  
	                  
         				
         				
         				
         				if( document.getElementById(&quot;fileOrg&quot;).checked )
     					{
         					
        					document.getElementById(&quot;individual&quot;).value=null;
         					document.getElementById(&quot;organisation&quot;).value=&quot;selected&quot;;	
     				
     				 if(document.getElementById(&quot;cusName12&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                	  {
                	 alert(&quot;Name should not be empty &quot;);
                	 return false;
                	  }
     				 
     				 
     				  var myVar=document.getElementById(&quot;cusName12&quot;).value;
                	  
                	  var hasNumber = /\d/;  
                	  
                	 if(hasNumber.test(myVar))
                		 {
                		 alert(&quot;Name should be alphabate&quot;);
                		 return false;
                		 
                		 }
     				 
     				 
                  
                  if(document.getElementById(&quot;dateOfBirth12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	 alert(&quot;datofbirth should not be empty &quot;);
            	 return false;
            	  }
                
                  if(document.getElementById(&quot;orgName&quot;).value == &quot;null&quot; || document.getElementById(&quot;orgName&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	 alert(&quot;orgName should not be empty &quot;);
            	 return false;
            	  }
                  var myVar1=document.getElementById(&quot;orgName&quot;).value;
            	  
            	  var hasNumber1 = /\d/;  
            	  
            	 if(hasNumber1.test(myVar1))
            		 {
            		 alert(&quot;Organization Name should be alphabate&quot;);
            		 return false;
            		 
            		 }
                  
                  if(document.getElementById(&quot;mobileNo12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	    alert(&quot;mobileNo should not be empty &quot;);
            	    return false;
            	  }
                  
               var number1= document.getElementById(&quot;mobileNo12&quot;).value;
                  
                  number1 = number1.trim();
                  if(number1 == &quot;&quot;) {
            	        window.alert(&quot;Mobile number should not be empty&quot;);
            	        number1.focus();
            	        return false;
            	    }

            	    if(number1.length != 10) {
            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
            	        number1.focus();
            	        return false;
            	    }
               
            	    
            	    var mNumber=document.getElementById(&quot;mobileNo12&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
            	    
            	    
            	  
                  /*if(document.getElementById(&quot;emailId12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	    alert(&quot;emailId should not be empty &quot;);
            	    return false;
            	  }*/
                  
                  
                  
                  
                  
                  
                 var x=document.getElementById(&quot;emailId12&quot;).value;
                 if(x != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; x!=null){ 
                  /* var atposition=x.indexOf(&quot;@&quot;);  
                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
                    alert(&quot;Please enter valid email id&quot;);  
                    return false;  
                    } */
                    x=x.trim();
                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
	         			alert(&quot;Please enter valid email id&quot;);
	         			return false;
	         		}
                 }
     					}	
         				
         				var corpdata=document.getElementById(&quot;corporate_1&quot;)
         				
         				var captchaOption = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;optionOfCaptcha&quot; , &quot;'&quot; , &quot;]:checked&quot;).val();
        				document.getElementById(&quot; , &quot;'&quot; , &quot;capchaRadioSel&quot; , &quot;'&quot; , &quot;).value=captchaOption;
         	    		//alert(&quot;captchaOption :: &quot;+captchaOption);
         				
         				var reqDataTmp = &quot; , &quot;'&quot; , &quot;cusName~dateOfBirth~mobileNo~captchaValue~cusName12~dateOfBirth12~mobileNo12~captchaValue12~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha
         				//alert(&quot;b4 check empty :&quot;+reqDataTmp);
    					var reqData = checkEmpty(reqDataTmp)
    					//alert(&quot;reqData is &quot;+reqData);
    					var reqDataEmail=document.getElementById(&quot;emailId&quot;)!=null?document.getElementById(&quot;emailId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    				    reqData+=&quot; , &quot;'&quot; , &quot;emailId=&quot; , &quot;'&quot; , &quot;+reqDataEmail+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
    				   // alert(&quot;reqDataEmail is &quot;+reqDataEmail);
    				    var reqDataEmail12=document.getElementById(&quot;emailId12&quot;)!=null?document.getElementById(&quot;emailId12&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    				    reqData+=&quot; , &quot;'&quot; , &quot;emailId12=&quot; , &quot;'&quot; , &quot;+reqDataEmail12+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
    				   //alert(&quot;reqDataEmail12 is &quot;+reqData);
    				    var encdata = getEncryptData(reqData)
    					$(&quot;#encStaticData&quot;).val(encdata);
    				    //alert(&quot;encStaticData is &quot;+encStaticData);

    				 var len = document.getElementById(&quot;paramLen&quot;).value;
    				    
    				  for(i=1,k=11;i&lt;=len;i++,k++)
                     	{ 
                	 var temp;
                	
					if(document.getElementsByName(&quot;outref&quot;+k).length >0){
						
					document.getElementsByName(&quot;outref&quot;+k)[0].value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
					
                	 } 

                }	
               
    				    
    				  /*   var revampcatId=document.getElementById(&quot;categoryId&quot;)!=null?document.getElementById(&quot;categoryId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				var revampInstId=document.getElementById(&quot;instituteId&quot;)!=null?document.getElementById(&quot;instituteId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				var revampInstutionId=document.getElementById(&quot;institutionId&quot;)!=null?document.getElementById(&quot;institutionId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampState=document.getElementById(&quot;state&quot;)!=null?document.getElementById(&quot;state&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCategoryName=document.getElementById(&quot;categoryName&quot;)!=null?document.getElementById(&quot;categoryName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampTransactioName=document.getElementById(&quot;transactionName&quot;)!=null?document.getElementById(&quot;transactionName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampcredit=document.getElementById(&quot;creditTxnCount&quot;)!=null?document.getElementById(&quot;creditTxnCount&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampDebit=document.getElementById(&quot;debitBranchCode&quot;)!=null?document.getElementById(&quot;debitBranchCode&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampDob=document.getElementById(&quot;dobName&quot;)!=null?document.getElementById(&quot;dobName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCreditBranch=document.getElementById(&quot;creditBranchCode&quot;)!=null?document.getElementById(&quot;creditBranchCode&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorp=document.getElementById(&quot;corpName&quot;)!=null?document.getElementById(&quot;corpName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorpAdd=document.getElementById(&quot;corpAddress&quot;)!=null?document.getElementById(&quot;corpAddress&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampInst=document.getElementById(&quot;instType&quot;)!=null?document.getElementById(&quot;instType&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampLogopath=document.getElementById(&quot;logoPath&quot;)!=null?document.getElementById(&quot;logoPath&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampLogoName=document.getElementById(&quot;logoName&quot;)!=null?document.getElementById(&quot;logoName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampIndividual=document.getElementById(&quot;individual&quot;)!=null?document.getElementById(&quot;individual&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCategory=document.getElementById(&quot;Category&quot;)!=null?document.getElementById(&quot;Category&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValid=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampTrans=document.getElementById(&quot;transactionRemarks&quot;)!=null?document.getElementById(&quot;transactionRemarks&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampSelected=document.getElementById(&quot;selectAccountFrom&quot;)!=null?document.getElementById(&quot;selectAccountFrom&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorporateNet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValidNotMand=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValidMand=document.getElementById(&quot;validateMandatory&quot;)!=null?document.getElementById(&quot;validateMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorporateInternet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				 */
         				
         				 
         				 
         				  //CR-9184 installment changes start
      	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;blank&quot; , &quot;'&quot; , &quot;)
      	            	{
      	                 	alert(&quot;please select payment method&quot;);
      		                return false;
      	                	}
      	                //CR-9184 installment changes end
      					 var val = document.getElementById(&quot;transactionRemarks&quot;).value;
         				 //alert(&quot;Remarks :: &quot;+val);alert(&quot;Remark boolean :: &quot;+val.match(/^[A-Za-z0-9 .,-_/()#\t\r\n\f]+$/i.test($.trim(val)));
         				//alert(&quot;Remark boolean :: &quot;+val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f]*$/)); //^[A-Za-z0-9@_/,&lt;>:^;()# .-]*$
         				if(val != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !(val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f ]*$/)) &amp;&amp; val != null &amp;&amp; val.trim()){
         					alert(&quot;Please enter Remarks in valid format!&quot;);
         					return false;
         				}
         				
 						var encmis = &quot; , &quot;'&quot; , &quot;categoryId~instituteId~institutionId~state~categoryName~dobName~corpName~corpAddress~instType~logoPath~logoName~organisation~individual~Category~transactionRemarks~CorporateInternetBanking~selectAccountFrom~validateMandatory~organisation~formInd~fileOrg&quot; , &quot;'&quot; , &quot;;
         				var reqData = checkEmpty(encmis)
         				var encdata = getEncryptData(reqData)
     					$(&quot;#encMis&quot;).val(encdata);
     				   
     				    
     				   /*  if(document.getElementById(&quot;instType&quot;)!=null)
     				    	{
         				    alert(&quot;before::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	document.getElementById(&quot;instType&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				    alert(&quot;After::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	} */
     				    
     				   /*  if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
     				    	{
     				    	document.getElementById(&quot;validateNotMandatory&quot;)=null;

     				    	//document.getElementById(&quot;validateNotMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
     				    	}
 */     				    
     				  /*  if(document.getElementById(&quot;validateMandatory&quot;)!=null)
				    	{
				    	document.getElementById(&quot;validateMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	} */
     				    
     				  /* if(document.getElementById(&quot;CorporateInternetBanking&quot;)!=null)
				    	{
				    	document.getElementById(&quot;CorporateInternetBanking&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	}  
				    	
				  	  if(document.getElementById(&quot;selectAccountFrom&quot;)!=null)
				    	{
				    	document.getElementById(&quot;selectAccountFrom&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	}  */

     				//alert(&quot;Hi &quot;+document.getElementById(&quot;validateNotMandatory&quot;).value);alert(&quot;2 &quot;+document.getElementById(&quot;organisation&quot;).value);
     				
     				   //removed from here
   		         
   		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
    				    
    				    
    				    
    				    
         				
         				if(!corpdata.checked)
         					{
         					alert(&quot;Please read and accept the Terms and Conditions&quot;);
         					return false;
         					}
    				
         				var capt=document.getElementById(&quot;captchaValue&quot;).value
         				
         				if(capt == &quot;null&quot; || capt == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
         					{
         					
         					alert(&quot;Please enter captcha value as shown in the image or from the audio&quot;);
         					return false;
         					}
         				else
         					{
         				//	alert(&quot;Am here!!&quot;);
         				//	alert(capt);
         					//	
         					var alphaNumExp = new RegExp(&quot;^[0-9a-zA-Z]+$&quot;);
         					 if(!document.getElementById(&quot;captchaValue&quot;).value.match(alphaNumExp)) {
								alert(&quot; , &quot;'&quot; , &quot;Please enter valid text as shown in the image or from the audio&quot; , &quot;'&quot; , &quot;);
								document.getElementById(&quot;captchaValue&quot;).value=&quot;&quot;;
								document.getElementById(&quot;captchaValue&quot;).focus();
								return false;
						      }else{
         							validateCaptcha(&quot; , &quot;'&quot; , &quot;paymentDetails&quot; , &quot;'&quot; , &quot;,capt);
         					  }
         					}
         				
     	                
         				// $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);   
       				    // $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				  
         				  
         				 //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
      	              //   $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
      	               
         				  
         				  //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		             // $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
      		             //alert(&quot;data is &quot;+document.getElementById(&quot;validateMandatory&quot;));
      		             
      		             
      		             	//CR-9184 installment changes start
					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;partial&quot; , &quot;'&quot; , &quot;)
					{
					   //alert(&quot;one&quot;);
						$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;installmentdetails.htm&quot; , &quot;'&quot; , &quot;);
						}
					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;full&quot; , &quot;'&quot; , &quot;))
						{
						//alert(&quot;two&quot;);
						$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;installmentdetailsconfirmpage.htm&quot; , &quot;'&quot; , &quot;);
						}
					//CR-9184 installment changes end
					else
					   {
						//alert(&quot;three&quot;);
					$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;confirmpayment.htm&quot; , &quot;'&quot; , &quot;);
					 }
				
       				//$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;confirmpayment.htm&quot; , &quot;'&quot; , &quot;);
        				
         				
         }
 
         	function validateCaptcha(formName, captchaValue) {
         		//img Aud Captcha
        		var captchaOption = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;optionOfCaptcha&quot; , &quot;'&quot; , &quot;]:checked&quot;).val();
        		document.getElementById(&quot; , &quot;'&quot; , &quot;capchaRadioSel&quot; , &quot;'&quot; , &quot;).value=captchaOption;
         		
         	      var captcha = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + captchaValue).val();
         	      
         	     captcha=captchaValue;
         	      
         	      var validateurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/validatecaptcha.htm&quot; , &quot;'&quot; , &quot;;
         	     
         	    var reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;; //img Aud Captcha 
         	      var errorurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/sessiontimeout.htm&quot; , &quot;'&quot; , &quot; ; 
         	    
         	       var successurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/confirmpayment.htm&quot; , &quot;'&quot; , &quot; ;
         	      
         	       var installmentDetails = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/installmentdetails.htm&quot; , &quot;'&quot; , &quot; ;
         	     
         	       var installmentDetailsConfirm = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/installmentdetailsconfirmpage.htm&quot; , &quot;'&quot; , &quot; ;
         	    //audio
         			if(captchaOption ==&quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;)
         				reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/audio.wav?bogus=&quot; , &quot;'&quot; , &quot;;
         	      
         	      $.ajax({
         	            type : &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
         	            dataType : &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;,
         	            url : validateurl,
         	          //  data : &quot; , &quot;'&quot; , &quot;captchaVal=&quot; , &quot;'&quot; , &quot; + captcha,//commented for img Aud Captcha
		    				data:jQuery.param({captchaVal:captcha,captchaOption:captchaOption}),
         	            success : function(data) { 
         	            	
         	            	
         	            	
         	                  if (data == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;) {
         	                	  
         	                	 $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#installmentParamKey&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	              		 $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); $(&quot; , &quot;'&quot; , &quot;#installmentParamKey&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	              		
         	              		 $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	                	  
         	                	 $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);   
        				    	 $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        				    	 $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	                //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	               // $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
             	                //$(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	                $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
             	               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
             				   $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        			
        		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
          			
           		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		              // $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        			
        		               //$(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		              // $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
          			
           		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);//imag and audio captcha
        		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#captchaValue12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#formInd&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);  
           		               $(&quot; , &quot;'&quot; , &quot;#corporate_1&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         	                	  
         	                 	//CR-9184 installment changes start
         	 					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;partial&quot; , &quot;'&quot; , &quot;)
         	 					{
         	 						$(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
      	                              action : installmentDetails
      	                        	});
         	 						}
         	 					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;full&quot; , &quot;'&quot; , &quot;))
         	 						{
         	 						 $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
        	                              action : installmentDetailsConfirm
        	                        });
         	 						 }
         	 					//CR-9184 installment changes end
         	 					else
         	 					   {
         	 						 $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
        	                              action : successurl
        	                        }); 
         	 						 
         	 					   }
         	 				
         	                 	
         	                    //   alert(&quot;data in captcha&quot;+document.getElementById(&quot;validateMandatory&quot;));

         	                    //  document.getElementById(&quot;validateMandatory&quot;)=null;
         	                    //alert(&quot;1111111111111111111111111111111111111111&quot;);
         	                       $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).submit();
         	                      
         	                  }else if(data ==&quot; , &quot;'&quot; , &quot;sessiontimeout&quot; , &quot;'&quot; , &quot;){
         	                        $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
         	                              action : errorurl
         	                        });   
         	                                                
         	                        $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).submit();
         	                  }else {

         	                        alert(&quot; , &quot;'&quot; , &quot;Please enter valid captcha as shown in the image or from the audio&quot; , &quot;'&quot; , &quot;);
         	                      //  $(&quot; , &quot;'&quot; , &quot;#captcha&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
         	                      $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
         	                       document.getElementById(&quot;captchaValue&quot;).value = &quot;&quot;;
         	                        document.getElementById(&quot;imageContainer&quot;).removeChild(
         	                    //    document.getElementById(&quot;captchaImage&quot;));
         	                        		document.getElementById(&quot;refreshImgCaptcha&quot;));//img Aud Captcha
         	                        var newImg = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
         	                      //  newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;captchaImage&quot;);
         	                      newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;refreshImgCaptcha&quot;);//img Aud Captch
         	                        newImg.setAttribute(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, reloadurl+ new Date().getTime());
         	                        newImg.setAttribute(&quot; , &quot;'&quot; , &quot;alt&quot; , &quot;'&quot; , &quot;, &quot;Captcha&quot;);
         	                        var div = document.getElementById(&quot; , &quot;'&quot; , &quot;imageContainer&quot; , &quot;'&quot; , &quot;);
         	                        div.appendChild(newImg);

         	                  }
         	            },
         	            error : function(data) {
         	                  alert(&quot; , &quot;'&quot; , &quot;error : &quot; , &quot;'&quot; , &quot; + data);
         	            }
         	     
         	      }); 
         	    // document.getElementById(&quot;validateMandatory&quot;)=null;
         	      
         	      
         	}  

     
         	function captchaRefresh()
         	{
         		$(&quot; , &quot;'&quot; , &quot;#captcha&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
                 document.getElementById(&quot;imageContainer&quot;).removeChild(
                             document.getElementById(&quot;captchaImage&quot;)); 
                             var reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/loadcaptcha.htm?&quot; , &quot;'&quot; , &quot;;
                 var newImg = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;captchaImage&quot;);
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, reloadurl+ new Date().getTime());
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;alt&quot; , &quot;'&quot; , &quot;, &quot;Captcha&quot;);
                 var div = document.getElementById(&quot; , &quot;'&quot; , &quot;imageContainer&quot; , &quot;'&quot; , &quot;);
                 div.appendChild(newImg);
         	}
         	
         	
         	
         	
         	function backInstitution()	{
         		document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		document.paymentDetails.submit();
         	}

         	function resetFeeParams(formName)	{

         		if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;) != null){
         			var validateMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;);		
         			if(validateMandatory.length==undefined)
         				validateMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;));
         				
         			for(i=0;i&lt;validateMandatory.length;i++){
         				
         				var validations=validateMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         				var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validations[0]);
         				
         				if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		  }

         		if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateNotMandatory&quot; , &quot;'&quot; , &quot;) != null)	{
         			
         			var validateNotMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;);		
         			if(validateNotMandatory.length==undefined)
         				validateNotMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;));
         				
         			for(i=0;i&lt;validateNotMandatory.length;i++){
         				
         				var validationsNotMand =validateNotMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         				var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validationsNotMand[0]);
         				
         				if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		} 
         		
         		$(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         		
         	}
         	
         	
         	function OpenExcel(path,name)
         	{
         	   //var wind=window.open(path+name,&quot;Report&quot;,&quot;toolbar=yes,scrollbars=yes,resizable=yes,top=0,left=0,menuBar=yes&quot;);
         		path=path+name;//alert(&quot;full : &quot;+path);
        		var link=document.createElement(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);
        		link.href = path;
        		link.download = path.substr(path.lastIndexOf(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;) + 1);
        		link.click();
         	}

         	
         	function getEncryptData(reqData)
         	{
         		var key = $(&quot;#eKeVal&quot;).val();
         		if(key != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
         		{
         			
         			return CryptoJS.AES.encrypt(reqData,key)
         		}
         	}

         	
         	function checkEmpty(reqData)
         	{
         		var validInput = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         		if(reqData.indexOf(&quot;~&quot;) > 0)
         		{
         			var arr = reqData.split(&quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot;);
         			for(var i=0;i&lt;arr.length;i++)
         			{ 
         				var temp;
         				if(document.getElementById(arr[i]) !=null)
         				 temp = document.getElementById(arr[i]).value;
         				if(temp == &quot;&quot;) {
         					validInput += arr[i]+&quot;=&quot;+&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;+&quot;&amp;&quot;;
         				}else {
         					validInput += arr[i]+&quot;=&quot;+temp+&quot;&amp;&quot;;
         				}
         			}
         			return validInput;
         		}
         	
         	
 
         
         
         	}  
         	
         	
         	function myFunction1()
         	{
         		
     			 var w = document.paymentDetails.Category.selectedIndex;
    			 var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
    			 var ins1=&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;;
    			 var st1=&quot; , &quot;'&quot; , &quot;Tamil Nadu&quot; , &quot;'&quot; , &quot;;
    	
    			 
    			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
    		    document.getElementById(&quot;institutionId&quot;).value=ins1;
    			document.getElementById(&quot;state&quot;).value=st1;
    	
    	
    	var selected_text = document.paymentDetails.Category.options[w].text;
    	
    	if (selected_text == &quot;--Select Category--&quot;) {
    		 
    		
    		 return false;
    		 
    	} else {
    		
    		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
    		
    		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
    	
    		
    		var fileMode=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;fileMode&quot; , &quot;'&quot; , &quot;);
    	
    		var paramName=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;paramName&quot; , &quot;'&quot; , &quot;);
    		
    		var optStr=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;optString&quot; , &quot;'&quot; , &quot;);
    	
    		var dupPayment = category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;dupPayment&quot; , &quot;'&quot; , &quot;);
    		
             //API    
    		if(fileMode==&quot; , &quot;'&quot; , &quot;Y&quot; , &quot;'&quot; , &quot; || fileMode==&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;)
    		{
    			
    		
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
    				var arr = paramName.split(&quot;-&quot;);
    				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
    				
    				
    				
    				
    				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
    				
    				var arr = paramName.split(&quot;-&quot;);
        			
        			         				
    				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
    				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
    				
    				
    				
    				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/filemode.htm&quot; , &quot;'&quot; , &quot;;
    				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				

    			
    			
    			}
    		else
    			{
    			
    				
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;  
    				
    				
    				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listcategory.htm&quot; , &quot;'&quot; , &quot;;
    				
    				
    				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				
    				return false; 
    			}
    		return true;
    		} 
    }
         	
         	
         	function restFeeParams(formName)	{
         		
         		var indOrOrgVal = &quot;&quot;; //def7 
         		if( $(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ){ //def7 
         			indOrOrgVal = &quot;fileOrg&quot;;
         		} 
         		
         	
         			 var selectedCaptcha = $(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha]:checked&quot; , &quot;'&quot; , &quot;).val();  //def 9
         	
         			
         		
          		 document.getElementById(&quot; , &quot;'&quot; , &quot;loginAudioCaptcha&quot; , &quot;'&quot; , &quot;).controls = false; //def 8
         		 document.getElementById(&quot; , &quot;'&quot; , &quot;paymentDetails&quot; , &quot;'&quot; , &quot;).reset();
         		document.getElementById(&quot; , &quot;'&quot; , &quot;loginAudioCaptcha&quot; , &quot;'&quot; , &quot;).controls = true; //def 8
         		// getUserSelImgCaptcha();
         		 
         		 if(indOrOrgVal == &quot; , &quot;'&quot; , &quot;fileOrg&quot; , &quot;'&quot; , &quot;){ //def7 
         			$(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).prop( &quot;checked&quot;, true );
         		 }else{
         			$(&quot; , &quot;'&quot; , &quot;#formInd&quot; , &quot;'&quot; , &quot;).prop( &quot;checked&quot;, true );
         		 }//def7 
         		 
         		
         		 if(selectedCaptcha == &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;){  //def 9
         			$(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha][value=AUD]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
         			getUserSelAudCaptcha();
         		 }else{
          			$(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha][value=IMG]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
         			getUserSelImgCaptcha();
            		 
         		 } //def 9
         		}
         	function backFeeParams(formName)	{
         		
         		 
         		var insName=&quot; , &quot;'&quot; , &quot;Educational&quot; , &quot;'&quot; , &quot;;
         		//var insID =&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;
         		document.getElementById(&quot; , &quot;'&quot; , &quot;instType&quot; , &quot;'&quot; , &quot;).value=insName;
         		//document.getElementById(&quot; , &quot;'&quot; , &quot;institutionId&quot; , &quot;'&quot; , &quot;).value=insID;
         		//document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		//document.paymentDetails.submit();
         		//Added by supriya for back button issue
         		var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listinstitution.htm&quot; , &quot;'&quot; , &quot;;
				
				
				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
				$(&quot;#paymentDetails&quot;).submit();
				//Added by supriya for back button issue
         	}
         	
         	 //Function to set the image captcha as default option on selection of individula/org radio button
         	function resetToImageCaptcha(){
         		if($(&quot; , &quot;'&quot; , &quot;input:radio[name=optionOfCaptcha]:checked&quot; , &quot;'&quot; , &quot;).val()==&quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;)
         	   {
         	   	$(&quot; , &quot;'&quot; , &quot;input:radio[name=optionOfCaptcha]&quot; , &quot;'&quot; , &quot;)[0].checked = true; 
               	getUserSelImgCaptcha();
         	   }
         	}
         	
         
	  




		
		
		
			
		
		
		 
        
            
                 © State Bank of India 
            
            
                Privacy Statement 
                 
                Disclosures
				 
				Terms of Use
            
        
		  		
		 
   
  
   $(document).ready(function() {
   var filename = window.location.href.substr(window.location.href.lastIndexOf(&quot;/&quot;)+1);
   
     switch(filename) {
	  case &quot;icollecthome.htm&quot;:   
	 $(&quot;#Link_1&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
    case &quot;paymenthistory.htm&quot;:   
	 $(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
    case &quot;faq.htm&quot;:    
    $(&quot;#Link_3&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
	 case &quot;customersupport.htm&quot;:    
   $(&quot;#Link_4&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    break;
	 case &quot;paymenthistorydatedetails.htm&quot;:
	$(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
	break;
	 case &quot;sendotp.htm&quot;:
	$(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
	break;
  default:
   $(&quot;#Link_1&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);    
  } 
   
});


   
		
		
		
	
	


 
	


	




   	
 
	
  
      
       




			
				
					
					
					
					
					Thank you for choosing SB Collect. As per RBI guidelines on cross-border payment transactions, maximum amount of Rs 25,00,000/- per transaction / per item can only be processed on this platform. Hence, this transaction is declined. On clicking &quot;OK&quot; you will be redirected to SB Collect Corporate home page. Inconvenience regretted.
					
					
					
					OK
					
				
			



function logout()
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;logout.htm&quot; , &quot;'&quot; , &quot;);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}

function callURL(url)
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}
function callLogout(url)
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}

/* function callMopsPage() {
	
	var domain=document.domain;
	var win = window.open(&quot; , &quot;'&quot; , &quot;https://&quot; , &quot;'&quot; , &quot;+domain+&quot; , &quot;'&quot; , &quot;/prelogin/mopsremittanceform.htm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);

//	var win = window.open(&quot; , &quot;'&quot; , &quot;https://www.onlinesbi.com/prelogin/mopsremittanceform.htm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);

	if (win) {
	    win.focus();
	}
} */
function callInst(url)
{
	  // alert(&quot; , &quot;'&quot; , &quot;callInst&quot; , &quot;'&quot; , &quot;+url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}
function callCat(url){
//	alert(&quot; , &quot;'&quot; , &quot;callact&quot; , &quot;'&quot; , &quot;);
	$(&quot; , &quot;'&quot; , &quot;#institutionform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#institutionform&quot; , &quot;'&quot; , &quot;).submit();
	
}
 function paySubmit(payflag)
	{
		//alert(&quot; , &quot;'&quot; , &quot;payflag&quot; , &quot;'&quot; , &quot;+payflag);
		var hourCheck = $(&quot; , &quot;'&quot; , &quot;#hourCheck&quot; , &quot;'&quot; , &quot;).val();
		var minCheck = $(&quot; , &quot;'&quot; , &quot;#minCheck&quot; , &quot;'&quot; , &quot;).val();
		
		var foreignCardTxnLimit = parseFloat(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		var amountTransfer      = parseFloat(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		
		if(((hourCheck==23 &amp;&amp; minCheck>=30) || (hourCheck==00 &amp;&amp; minCheck&lt;=30) ) &amp;&amp; (payflag==&quot; , &quot;'&quot; , &quot;SBDEBIT&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;OTHERDCARD&quot; , &quot;'&quot; , &quot;||payflag==&quot; , &quot;'&quot; , &quot;CREDITCARD&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;PREPAID&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;RUPAYCARD&quot; , &quot;'&quot; , &quot;))
		{
		 	alert(&quot;This payment mode is not available between 23:30 hours IST and 00:30 hours IST&quot;);
		}else if(((hourCheck == 22 &amp;&amp; minCheck >= 30) || (hourCheck == 23 &amp;&amp; minCheck &lt;= 30) ) &amp;&amp; (payflag == &quot; , &quot;'&quot; , &quot;UPI&quot; , &quot;'&quot; , &quot;))
		{
		 	alert(&quot;This payment mode is not available between 22:30 hours IST and 23:30 hours IST&quot;);
		} else {
			
			$(&quot; , &quot;'&quot; , &quot;#payflag&quot; , &quot;'&quot; , &quot;).val(payflag);

			if( payflag==&quot; , &quot;'&quot; , &quot;SBDEBIT&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;OTHERDCARD&quot; , &quot;'&quot; , &quot;||payflag==&quot; , &quot;'&quot; , &quot;CREDITCARD&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;PREPAID&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;RUPAYCARD&quot; , &quot;'&quot; , &quot;) {
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fsspaymentgateway.htm&quot; , &quot;'&quot; , &quot;);
			}else if(payflag == &quot; , &quot;'&quot; , &quot;UPI&quot; , &quot;'&quot; , &quot;){
			$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;upipayment.htm&quot; , &quot;'&quot; , &quot;);
			
		
			}else {
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;suvidhapayment.htm&quot; , &quot;'&quot; , &quot;);
			}
			
			if(payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; &amp;&amp; amountTransfer > foreignCardTxnLimit){
				$(&quot; , &quot;'&quot; , &quot;#transactionModal&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
			}else{
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).submit();
			}		
		}
	}
 function submitVPA()
 { //alert(&quot; , &quot;'&quot; , &quot;submitVPA&quot; , &quot;'&quot; , &quot;);
 	/* var validator = $(&quot;#frmupi&quot;).validate (
 	{
 		focusCleanup: true,
 		onkeyup: false,
 		onblur: false,
 		onfocusout:false,
 		rules:{
 			vpa:{ required : true,
 				vpacheck : true}
 		},
 		messages: {
 			vpa :{
 				required :&quot;Please enter VPA&quot;,
 				vpacheck:&quot;You have entered an invalid VPA!&quot;
 				}
 		}
 	}); */
 	
 /* 	if (validator.form()) { */
 	//	doencrypt();
 		$(&quot; , &quot;'&quot; , &quot;form#frmupi&quot; , &quot;'&quot; , &quot;).attr({
 			action : &quot; , &quot;'&quot; , &quot;validateVPA.htm&quot; , &quot;'&quot; , &quot;
 		});	
 		$(&quot; , &quot;'&quot; , &quot;#frmupi&quot; , &quot;'&quot; , &quot;).submit();
 	//}
 }




$(document).ready(function () {
	
	//Disable full page
/*   $(&quot;body&quot;).on(&quot;contextmenu&quot;,function(e) {
      return false;
  }); */
  
  $(this).bind(&quot;contextmenu&quot;, function(e) {
      e.preventDefault();
  });
	
  function disableBack() { window.history.forward() }
  
  window.onload = disableBack();
  window.onpageshow = function(evt) { if (evt.persisted) disableBack() }

    
  $(document).keydown(function (e) {
        return (e.which || e.keyCode) != 116;
  });
    
});




    $(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).select2();



/html[1]/body[1]/section[@class=&quot;section_div&quot;]/div[@class=&quot;col-lg-12  col-md-12 col-sm-12 col-12 content_section&quot;]&quot;) or . = concat(&quot;
	
	
		

		
		
		
			




  

  
  
	 	
	
		  
	
	 	
	
		
	
	
				 
			 
				
					 SB Collect 
				
				
								
			 
			
			 
			

				
				
				
				
				  
					
					  Home 
					
					
					   Transaction
									History
							
					
					  FAQ&quot; , &quot;'&quot; , &quot;s
					
					
					  Customer Support
					
				  
				

			
			
			 
	
		
	
	
  
  

 

		
		
		
			












 


SB Collect












#loginAudioCaptcha{
	width:100% !important;
}
#refreshImgCaptcha
{
width:125px;
 margin-top: -8px;
}

@media only screen 
and (min-device-width : 768px) 
and (max-device-width : 1024px)  {
#refreshImgCaptcha
{
width:120px !important;
}
}
 
 
 
 $(document).ready(function() {
		getUserSelImgCaptcha();
		
});

	// Opera 8.0+
 var isOpera = (!!window.opr &amp;&amp; !!opr.addons) || !!window.opera || navigator.userAgent.indexOf(&quot; , &quot;'&quot; , &quot; OPR/&quot; , &quot;'&quot; , &quot;) >= 0;
 // Firefox 1.0+
	var isFirefox = typeof InstallTrigger !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;;
 // Safari 3.0+ &quot;[object HTMLElementConstructor]&quot; 
 var isSafari = /constructor/i.test(window.HTMLElement) || (function (p) { return p.toString() === &quot;[object SafariRemoteNotification]&quot;; })(!window[&quot; , &quot;'&quot; , &quot;safari&quot; , &quot;'&quot; , &quot;] || (typeof safari !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; safari.pushNotification));
 // Internet Explorer 6-11
 var isIE = /*@cc_on!@*/false || !!document.documentMode;
 // Edge 20+
 var isEdge = !isIE &amp;&amp; !!window.StyleMedia;
 // Chrome 1 - 71
 var isChrome = !!window.chrome &amp;&amp; (!!window.chrome.webstore || !!window.chrome.runtime);
 // Blink engine detection
 var isBlink = (isChrome || isOpera) &amp;&amp; !!window.CSS;
	//window.onload = function(){ 
	 
		//getUserSelImgCaptcha();
		
//	} 
  
  function getUserSelImgCaptcha() {
 	
 	if(null!=document.paymentDetails.captchaValue){//audio
 		document.paymentDetails.captchaValue.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; //audio
     	document.paymentDetails.capOption.value = &quot; , &quot;'&quot; , &quot;IMG&quot; , &quot;'&quot; , &quot;;
     //	document.getElementById(&quot; , &quot;'&quot; , &quot;noAudImgSelection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;audioselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot; , &quot;'&quot; , &quot;audelcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 		document.getElementById(&quot;refreshImgCaptcha&quot;).src=&quot; , &quot;'&quot; , &quot;/sbicollect/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;+(new Date().getTime());

 	}
 	
	}


 function getUserSelAudCaptcha() {
 	document.paymentDetails.captchaValue.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
 	document.paymentDetails.capOption.value = &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;;
 	//document.getElementById(&quot; , &quot;'&quot; , &quot;noAudImgSelection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
 	document.getElementById(&quot; , &quot;'&quot; , &quot;imgselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;imgselcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;audioselection&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
		document.getElementById(&quot; , &quot;'&quot; , &quot;audelcaptcha&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
		if (isIE == true){
			document.getElementById(&quot; , &quot;'&quot; , &quot;IEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot; , &quot;'&quot; , &quot;othrIEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
		}else {
			document.getElementById(&quot; , &quot;'&quot; , &quot;othrIEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot; , &quot;'&quot; , &quot;IEAud&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
			document.getElementById(&quot;loginAudioCaptcha&quot;).src = &quot; , &quot;'&quot; , &quot;/sbicollect/audio.wav?bogus=&quot; , &quot;'&quot; , &quot; + new Date().getTime();
				
		}
	}
 
 function refreshImg()
 {
 	if(null!=document.getElementById(&quot;refreshImgCaptcha&quot;)){
 document.getElementById(&quot;refreshImgCaptcha&quot;).src=&quot; , &quot;'&quot; , &quot;/sbicollect/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;+(new Date().getTime());
 
 	}
 }   
 
 



	
		 
		 
			 
			 
			
		 
		
			 
			 
			 
			
		    
		    
		
		
		 
		 
		 
		 
		
		
		
		
 
		 
		

		
		 
		 
		
		

		
	    


		 
		  
		
		
		 
		
	
             
        
        
        
        
        
      






		
		 
		
			
				
				
					Payment Progress
					
						Select Payee
						Enter Payment Details
						Verify Payment Details
						Complete Payment
						Print Receipt
					
				
			
		
		
		
			
				 
				
				 BDU CHIEF HOSTEL ADMINISTRATOR  |  CENTRAL OFFICE FOR HOSTEL ADMINISTRATION, BHARATHIDASAN UNIVERSITY, , TRICHY-620024
				
	
			
			

			
				


					
						
							Enter Payment Details
							
								
									
									
									
									Payment Category*:
									
									
								
								
								
								
								
									 
									
									
										 --Select any Category-- 
										
											
												

													GUEST FEE
												
												
											
										
											
												
												

													
														HOSTEL ADMISSION FEE 
												
											
										
											
												
												

													
														HOSTEL MESS FEES 
												
											
										
									GUEST FEE
									
									
								
							
						
	

          
          
          	
			
									
								
			
			
				
				
				
				
				
				
				
		  	
			
						
					
						
					
						
						
							
							



NAME *










						
							
							



COURSE/PURPOSE *










						
							
							



DEPARTMENT *










						
							
							



NUMBER OF DAYS *










						
							
							



FROM (DD/MM/YYYY) *










						
							
							



TO (DD/MM/YYYY) *










						
							
							



MOBILE *










						
							
							



AMOUNT (In Rs,) *










						
						

						
							


								
									Remarks : 
								
								
									 
								
							
						
					
				
					
				
				
					Please confirm the detail before proceeding for payment. Save a copy of e-receipt for your future reference.
				
				
					For any queries please contact Office 0431-2407038
				
				
				


						
							Enter Your Details

							
								
									 Individual
								
								
									 Organisation
										/ Corporate
								
							
						



						
							
								
									
										Name * :
										
									
									
										
									
									
									
									
										
										Date of
											Birth * :
										
										
										 
										
									
										
									
									
										
											
											
										 
									
								
							
							
								
									
										Mobile No * :
										
									
									
										
										On successful completion of
											payment,you will receive the transaction reference number on
											this mobile number
									
									
									
										Email ID :
										
									
									
										
										On successful completion of payment,
											you will receive the transaction reference number on this
											email ID
									
								
							

						

						

							

								
									Your Name * :
									
								
								
									
								

								


								
									Organisation
										Name * :
									
								
								
									
								


							
							

								
								
									
									Date of
										Incorporation * :
									
									
									
									
								
										
								
								

									
										 
									
								

								


								
									Mobile No * :
									
								
								
									
									On successful completion of payment,you
										will receive the transaction reference number on this mobile
										number
								


							
							

								
									Email Id :
									
								
								
									
									On successful completion of payment,
										you will receive the transaction reference number on this
										email ID

								

								

							

						
				

						
							
								
									
										 I
											have read and agreed to the Terms
												&amp; Conditions

										
									
								
							
						
					


						
						
							
								
									Enter the text as shown in the
											image
										*:
									

								
								
									Enter the text as from audio *

								
								
								
									
										Select one of the Captcha options
											*
									
                                        
											Image
											Captcha
										
										 
											Audio
											Captcha
										
									

								
								
						
									 
											
											
												
											
								
								
									
											
											
										
										
										
										  
										
								
								
							
						
				
						
							
								
								
								Back
								
								
								
								
								Reset
								
								Next
								
								
							
						
					

				
			
			
			
		
		

		
			
				
					

					
					
						X
						Terms and Conditions
						  
						 
						
								Corporate Customer: Firm/Company/Institution (F/C/I) collecting payment from their beneficiaries.
								User: The beneficiary making a payment to F/C/I for the services/goods availed.
								Bank shall not be responsible, in any way, for the quality or merchantability of any product/merchandise or any of the services related thereto, whatsoever, offered to the User by the Corporate Customer. Any disputes regarding the same or delivery of the Service or otherwise will be settled between Corporate Customer and the User and Bank shall not be a party to any such dispute. Any request for refund by the User on any grounds whatsoever should be taken up directly with the Corporate Customer and the Bank will not be concerned with such a request.
								Bank takes no responsibility in respect of the services provided and User shall not be entitled to make any claim against the Bank for deficiency in the services provided by the Corporate Customer.
								The User shall not publish, display, upload or transmit any information prohibited under Rule 3(2) of the Information Technology (Intermediaries guidelines) Rules, 2011.
								In case of non-compliance of the terms and conditions of usage by the User, the Bank has the right to immediately terminate the access or usage rights of the User to the computer resource of the Bank and remove the non-compliant information.
							
					
					
					
						Ok
					
				
			
		

		
	








         $(document).ready(function(){
        	// $(function() {  
        	 //$.datepicker.setDefaults({
        		//   inline: true,
        		 //  showOn: &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;,
        		  // buttonImageOnly: true,
        		  // buttonImage: &quot; , &quot;'&quot; , &quot;/sbijava/sbcollectrevamp/images/calendar_icon.png&quot; , &quot;'&quot; , &quot;,
        		 //  buttonText: &quot; , &quot;'&quot; , &quot;Calendar&quot; , &quot;'&quot; , &quot;
        		  
        		//   }); 
        	// $(&quot;#dateOfBirth&quot;).datepicker({ 
        		// dateFormat: &quot;dd/mm/yy&quot;,
        		// yearRange: &quot;-150:+0&quot;,
        		    
        		// maxDate: new Date()
        		// }).val()          
        		//  var changeMonth = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot; ); 
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeMonth&quot;, true ); 
        		// var changeYear = $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot; );
        		// $(&quot;#dateOfBirth&quot; ).datepicker( &quot;option&quot;, &quot;changeYear&quot;, true );
        		// $( &quot;#dateOfBirth&quot; ).datepicker({ appendText: &quot;(dd/mm/yy)&quot; });
        	// });
        		 
        	 
	   $(&quot;#ForOrganisation&quot;).hide();
	   
        $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).click(function(){
           if($(this).attr(&quot;value&quot;)==&quot;formInd&quot;){
               $(&quot;#ForOrganisation&quot;).hide();    
               $(&quot;#ForIndividual&quot;).show();
              //Added as part of Reset defect
                    $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot;#dateOfBirth12&quot;).datepicker(&quot; , &quot;'&quot; , &quot;setDate&quot; , &quot;'&quot; , &quot;, null); 
                    $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                    $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();//def-6
                    resetToImageCaptcha();
                                   
                }
          else if ($(this).attr(&quot;value&quot;)==&quot;fileOrg&quot;){
               $(&quot;#ForIndividual&quot;).hide();
              $(&quot;#ForOrganisation&quot;).show();
			   //$(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                   $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                   $(&quot;#dateOfBirth&quot;).datepicker(&quot; , &quot;'&quot; , &quot;setDate&quot; , &quot;'&quot; , &quot;, null);
                   $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus();
                   $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus(); 
                   $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).prop(&quot;removed&quot;, true).focus(); //def-6
                   resetToImageCaptcha();
      }
      });
	   //$(&quot;#exampleModalLong&quot;).modal(&quot;show&quot;);
	  });
        
// Somnath made change starts here
	//$(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).datepicker({
      //             format:&quot; , &quot;'&quot; , &quot;dd/mm/yyyy&quot; , &quot;'&quot; , &quot;,
        //      }).datepicker(&quot;setDate&quot;,&quot; , &quot;'&quot; , &quot;now&quot; , &quot;'&quot; , &quot;);
// Somnath made change end here
      //  $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).datepicker({
            //        format:&quot; , &quot;'&quot; , &quot;dd/mm/yyyy&quot; , &quot;'&quot; , &quot;,
             //   }).datepicker(&quot;setDate&quot;,&quot; , &quot;'&quot; , &quot;now&quot; , &quot;'&quot; , &quot;);
/*         
        $(function() {
        	 $(&quot;#dateOfBirth&quot;).datepicker({  maxDate: new Date() });
         });
         
         
         
         $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).datepicker({
            format: &quot;dd/mm/yyyy&quot;,
           	autoclose: true,
           	orientation: &quot;top&quot;,
          	endDate: &quot;today&quot;

       });
         
         
        $(&quot;.dateOfBirth&quot;).datepicker({
            maxDate: &quot;-1d&quot;, 
     		minDate: new Date(2007, 6, 12)
     });
         
*/         
//	Somnath made change starts here 
		[&quot;dateOfBirth&quot;,&quot;dateOfBirth12&quot;].forEach(function(val) { 
			$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+val).datepicker({autoclose:true});
		});
// 	Somnath made change starts ends 
         
         function selectCategory(category)
         {
           //alert(&quot;Entered into select category onchange&quot;);
         	var w = document.paymentDetails.Category.selectedIndex;
         	
        			  
         			  
         			 var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
         			 var ins1=&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;;
         			 var st1=&quot; , &quot;'&quot; , &quot;Tamil Nadu&quot; , &quot;'&quot; , &quot;;
         	
         			 
         			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
         			document.getElementById(&quot;institutionId&quot;).value=ins1;
         			document.getElementById(&quot;state&quot;).value=st1;
         	
         	
         	var selected_text = document.paymentDetails.Category.options[w].text;
         	
         	if (selected_text == &quot;--Select Category--&quot;) {
        		 return false;
         		 
         	} else {
         		
         		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
         		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
         		         		
         		var fileMode=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;fileMode&quot; , &quot;'&quot; , &quot;);
         		
         		var paramName=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;paramName&quot; , &quot;'&quot; , &quot;);
         		
         		var optStr=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;optString&quot; , &quot;'&quot; , &quot;);
         		
         		var dupPayment = category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;dupPayment&quot; , &quot;'&quot; , &quot;);
         		
                  //API    
         		if(fileMode==&quot; , &quot;'&quot; , &quot;Y&quot; , &quot;'&quot; , &quot; || fileMode==&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;)
         		{
         			
         			
         				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
         				var arr = paramName.split(&quot;-&quot;);
         				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
         				
       				
         				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
         				
         				var arr = paramName.split(&quot;-&quot;);
             			
             			         				
         				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
         				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
         				
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateNotMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
         				document.getElementById(&quot;validateMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				//document.getElementById(&quot;controls&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;formInd&quot;) !=null)
         				document.getElementById(&quot;formInd&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;fileOrg&quot;)!=null)
         				document.getElementById(&quot;fileOrg&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				
         				if(document.getElementById(&quot;corporate_1&quot;)!=null) 
         					document.getElementById(&quot;corporate_1&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				  
         				var onloadenc=&quot; , &quot;'&quot; , &quot;numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~firstkey~secondkey~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha&quot; , &quot;'&quot; , &quot;
         					var reqData = checkEmpty(onloadenc);
         					var encdynamicdata = getEncryptData(reqData);
        					$(&quot;#enconload&quot;).val(encdynamicdata);
         				
         				
         				
         				
         				
         				   $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				   $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
         				
         				   $(&quot; , &quot;'&quot; , &quot;#numberofkeyfield&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		              // $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
         				
      		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#fileMode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#encData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				
      		             $(&quot; , &quot;'&quot; , &quot;#encStaticData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#encdynamicData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#encKey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		              // $(&quot; , &quot;'&quot; , &quot;#encKeyVal&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
         				
    		               $(&quot; , &quot;'&quot; , &quot;#encMis&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref11&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#outref13&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               //$(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#outref14&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
    		               
    		               $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;); //audio image captcha
    		               $(&quot; , &quot;'&quot; , &quot;#Ur02H&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
    		              // $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
       				
         				
         				
         				
    				    var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/filemode.htm&quot; , &quot;'&quot; , &quot;;
         				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
         				$(&quot;#paymentDetails&quot;).submit();
       					return false;
 
         			}
         		else
         			{
         			document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;
         			
         			var onloadencCatList=&quot; , &quot;'&quot; , &quot;numberofkeyfield~categoryNameSel~categoryId~instituteId~institutionId~state~categoryName~fileMode~secondkey~firstkey~transactionName~creditTxnCount~debitBranchCode~netbankingflag~branchflag~creditcardflag~otherdebitflag~encData~dobName~creditBranchCode~paramLen~corpName~corpAddress~instType~logoPath~logoName~individual~organisation~encStaticData~encdynamicData~encMis~Category~outref11~outref12~outref13~outref14~transactionRemarks~cusName~dateOfBirth~mobileNo~emailId~cusName12~orgName~dateOfBirth12~mobileNo12~emailId12~captchaValue~Ur02H~selectAccountFrom~categoryNameSel~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha
     					var reqData = checkEmpty(onloadencCatList);
     					var encdynamicdata = getEncryptData(reqData);
    					  $(&quot;#encData&quot;).val(encdynamicdata);
     				
     				   $(&quot; , &quot;'&quot; , &quot;#numberofkeyfield&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		              $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
     				
  		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#fileMode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#secondkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#firstkey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		             $(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               //$(&quot; , &quot;'&quot; , &quot;#encData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		             $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
  		               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
     				
  		               $(&quot; , &quot;'&quot; , &quot;#encStaticData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#encdynamicData&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		              // $(&quot; , &quot;'&quot; , &quot;#encKey&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		              // $(&quot; , &quot;'&quot; , &quot;#encKeyVal&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
     				
		               $(&quot; , &quot;'&quot; , &quot;#encMis&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref11&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		              $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#outref13&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#outref14&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               $(&quot; , &quot;'&quot; , &quot;#orgName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
   				
		               
		               $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#Ur02H&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);//img Aud Captcha
		               $(&quot; , &quot;'&quot; , &quot;#categoryNameSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         			
         			
         			
         			
         			
         			
 				
         				  
      				
         				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listcategory.htm&quot; , &quot;'&quot; , &quot;;
    				    $(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
         				$(&quot;#paymentDetails&quot;).submit();
       				    return false; 
         			}
         		return true;
         		} 
         }

       
         
          function validateAndSubmitFeeParams(formName)
           {
       	  if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;) != null) {
        		
               //alert(&quot;inside validateManndatory data is &quot;+document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;));
        						var validateMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;);
        						
        						if(validateMandatory.length==undefined)
        							validateMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;));
        							
       							
        						for(i=0;i&lt;validateMandatory.length;i++) {
        							
        							var validations=validateMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         							var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validations[0]);

        							//Start of Code for Employee and Employer Contribution validation 
        							if(document.getElementById(&quot; , &quot;'&quot; , &quot;Category&quot; , &quot;'&quot; , &quot;))
        							{
        								 if(document.getElementById(&quot; , &quot;'&quot; , &quot;Category&quot; , &quot;'&quot; , &quot;).value ==&quot; , &quot;'&quot; , &quot;C005942&quot; , &quot;'&quot; , &quot;)	{
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Employee Contribution&quot; , &quot;'&quot; , &quot;){
        											employerContribution = controlObj.value;
        											employerId = validations[0];
        											}
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Employer Contribution&quot; , &quot;'&quot; , &quot;){
        											employeeContribution = controlObj.value
        											employeeId=validations[0];
        											}
        										
        										if(validations[1] ==&quot; , &quot;'&quot; , &quot;Number of Employees&quot; , &quot;'&quot; , &quot;) {
        											
        											noOfEmp=controlObj.value;
        											if(noOfEmp*3 != employerContribution){
        												alert(&quot;Enter correct value for Employee Contribution&quot;);
        												document.getElementById(employerId).value=&quot;&quot;;
        												return false;
        											}
        											
        											if(noOfEmp*6 != employeeContribution){
        												alert(&quot;Enter correct value for Employer Contribution&quot;)
        												document.getElementById(employeeId).value=&quot;&quot;;
        												return false;
        											}
        										}
        								
        								} 
        							}
         					//End of Code for Employee and Employer Contribution validation 
         					
         					if(controlObj.value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
         						if(validations[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         							alert(validations[3]);
         						else{
         					
         							if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1)
         								alert(&quot;Select &quot; + validations[1]);
         							else
         								alert(&quot;Enter &quot; + validations[1]);
         						}
         						controlObj.focus();
         						return false
         					}
         					
         				  //CR-9184 installment changes start
         	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;blank&quot; , &quot;'&quot; , &quot;)
         	            	{
         	                 	alert(&quot;please select payment method&quot;);
         		                return false;
         	                	}
         	                //CR-9184 installment changes end
         	
         					if (validations[2]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[2]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         					{

         					if(controlObj.value!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; controlObj.value.search(validations[2])!=0){
         						if(validations[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validations[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
         							if(validations[2] == &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,75}$)&quot; , &quot;'&quot; , &quot;)
         								alert(validations[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validations[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validations[3]); 
         							}
         						}
         						else{
         								alert(&quot;Enter &quot; + validations[1]);
         						}		
         						controlObj.focus();
         						return false
         					}
         					}
         					
         				}  // for loop complete
         				
         				
         			}	
         			
         			if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateNotMandatory&quot; , &quot;'&quot; , &quot;) != null){
         				var validateNotMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;);
         				
         				
         				if(validateNotMandatory.length==undefined)
         					validateNotMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;));
         					
         				for(i=0;i&lt;validateNotMandatory.length;i++){
         					
         					var validationsNotMand =validateNotMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         					var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validationsNotMand[0]);
                 			//23032023 date issue - regex changed to accept date and month in 01 - 09
         					if(controlObj.value ==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2] != &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2] != &quot; , &quot;'&quot; , &quot;(^(\\d{1,20})(\\.[0-9][0-9])?$)&quot; , &quot;'&quot; , &quot; &amp;&amp; validationsNotMand[2]!=&quot; , &quot;'&quot; , &quot;^(0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](0[1-9]|[1-9]|1[012])[- /.](19|20)\\d\\d$&quot; , &quot;'&quot; , &quot; &amp;&amp;  validationsNotMand[2]!=&quot; , &quot;'&quot; , &quot;^(0[1-9]|[1-9]|1[012])[- /.](0[1-9]|[1-9]|[12][0-9]|3[01])[- /.](19|20)\\d\\d$&quot; , &quot;'&quot; , &quot;){
         						alert(validationsNotMand[3]);
         						return false;
         					}
         					if(controlObj.value!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; controlObj.value.search(validationsNotMand[2])!=0){
         						if(validationsNotMand[3]!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;&amp;&amp; validationsNotMand[3]!=&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;)
         						{
         							if(validationsNotMand[2] == &quot; , &quot;'&quot; , &quot;(^[0-9a-zA-Z&amp;_. /@-]{1,76}$)&quot; , &quot;'&quot; , &quot;)
         								alert(validationsNotMand[3]);
         							else{
         								if(controlObj.value > 5000000000)
         								  alert(validationsNotMand[1] +&quot; exceeds transaction Limit&quot;)
         								else
         								  alert(validationsNotMand[3]); 
         							}
         							controlObj.focus();
         							return false;
         						}
         					}
         					
         					
         				}
         		  	}
         			
         			
         			
         			 var len = document.getElementById(&quot;paramLen&quot;).value;
                    
                     var reqdynamicData = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                 	var reqrefData=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                      for(i=1,k=11;i&lt;=len;i++,k++)
                     	{
                     
                     	 var temp;
                     	
  						if(document.getElementsByName(&quot;outref&quot;+k).length >0){ 
 						 temp = document.getElementsByName(&quot;outref&quot;+k)[0].value; 
 						reqrefData = &quot;outref&quot;+k+&quot;=&quot;+temp+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
 						
 						reqdynamicData+=reqrefData;
 					
                     	 } 

                     }
                    
                      
                      if(document.getElementById(&quot;installment&quot;)!=null)
               		{
               		var reqInstallment=document.getElementById(&quot;installment&quot;).value;

               		reqdynamicData+=&quot; , &quot;'&quot; , &quot;installment=&quot; , &quot;'&quot; , &quot;+reqInstallment+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
  					} 

                      
                      
                      var reqtransactionName=document.getElementById(&quot;transactionName&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;transactionName=&quot; , &quot;'&quot; , &quot;+reqtransactionName+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                        
                     
                      
                      var reqcreditTxnCount=document.getElementById(&quot;creditTxnCount&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditTxnCount=&quot; , &quot;'&quot; , &quot;+reqcreditTxnCount+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqdebitBranchCode=document.getElementById(&quot;debitBranchCode&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;debitBranchCode=&quot; , &quot;'&quot; , &quot;+reqdebitBranchCode+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                  
                      var reqcreditBranchCode=document.getElementById(&quot;creditBranchCode&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditBranchCode=&quot; , &quot;'&quot; , &quot;+reqcreditBranchCode+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      var reqinstallmentParamKey=document.getElementById(&quot;installmentParamKey&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;installmentParamKey=&quot; , &quot;'&quot; , &quot;+reqinstallmentParamKey+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      
                      
                                       
                      var reqcategoryId=document.getElementById(&quot;categoryId&quot;).value;
                     
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;categoryId=&quot; , &quot;'&quot; , &quot;+reqcategoryId+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  	 
                      
                      
                      var reqcategoryName=document.getElementById(&quot;categoryName&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;categoryName=&quot; , &quot;'&quot; , &quot;+reqcategoryName+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqnetbankingflag=document.getElementById(&quot;netbankingflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;netbankingflag=&quot; , &quot;'&quot; , &quot;+reqnetbankingflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                  
                      
                      var reqbranchflag=document.getElementById(&quot;branchflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;branchflag=&quot; , &quot;'&quot; , &quot;+reqbranchflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      
                      var reqcreditcardflag=document.getElementById(&quot;creditcardflag&quot;).value;
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;creditcardflag=&quot; , &quot;'&quot; , &quot;+reqcreditcardflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      var reqotherdebitflag=document.getElementById(&quot;otherdebitflag&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;otherdebitflag=&quot; , &quot;'&quot; , &quot;+reqotherdebitflag+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                    
                          					
                  	
                      var reqcontrols=document.getElementById(&quot;controls&quot;).value; 
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;controls=&quot; , &quot;'&quot; , &quot;+reqcontrols+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                     
                      //hidden fields ends
                      var reqparamLen = document.getElementById(&quot;paramLen&quot;).value;
                      reqdynamicData+=&quot; , &quot;'&quot; , &quot;paramLen=&quot; , &quot;'&quot; , &quot;+reqparamLen+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
                      
                      //alert(&quot;reqdynamicData is &quot;+reqdynamicData);
                      //var reqData = checkEmpty(reqdynamicData)
                      var encdynamicdata = getEncryptData(reqdynamicData)
  					  $(&quot;#encdynamicData&quot;).val(encdynamicdata);
                      
                  	
              		 
                     	
                      
                      var e = document.getElementById(&quot;Category&quot;);
                      
                     
                      var strUser = e.options[e.selectedIndex].text;
                      document.getElementById(&quot;categoryName&quot;).value=strUser;
                      
                    
                     
                     var e = document.getElementById(&quot;Category&quot;);
                     var strUser = e.options[e.selectedIndex].text;
                    
                   
                    
                    document.paymentDetails.categoryId.value =e.options[e.selectedIndex].value;
                    var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
                    
                    var catsel=&quot; , &quot;'&quot; , &quot;GUEST FEE&quot; , &quot;'&quot; , &quot;;
                    
                    
                    
                    var selCategory=document.paymentDetails.categoryId.value;
                    
                    
                    if(selCategory == catsel)
                    	{
                    	 document.paymentDetails.categoryId.value=categoryId1;
                    	}
                    
                  
         				var dynaDobName=document.getElementById(&quot;dobName&quot;).value;  //EMRO
         				
         				
         				if( document.getElementById(&quot;formInd&quot;).checked)
         					{
         					
         					document.getElementById(&quot;individual&quot;).value=&quot;selected&quot;;
         					document.getElementById(&quot;organisation&quot;).value=null;
         					
         				 if(document.getElementById(&quot;cusName&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	                	  {
	                	 alert(&quot;Name should not be empty &quot;);
	                	 return false;
	                	  }
         				 
 				 
	                	  var myVar=document.getElementById(&quot;cusName&quot;).value;
	                	  
	                	  var hasNumber = /\d/;  
	                	 
	                	 if(hasNumber.test(myVar))
	                		 {
	                		 alert(&quot;Name should be alphabet&quot;);
	                		 return false;
	                		 
	                		 }
	                  
	                 if(document.getElementById(&quot;dateOfBirth&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	 {
	            	 alert(&quot;datofbirth should not be empty &quot;);
	            	 return false;
	            	  }
	                
	                
	                
	                  if(document.getElementById(&quot;mobileNo&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	  {
	            	    alert(&quot;mobileNo should not be empty &quot;);
	            	    return false;
	            	  }
	                  
	                  var mNumber=document.getElementById(&quot;mobileNo&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
	                
	                  
	                  
	                  /*if(document.getElementById(&quot;emailId&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
	            	  {
	            	    alert(&quot;emailId should not be empty &quot;);
	            	    return false;
	            	  }*/
	                  
	                  var x=document.getElementById(&quot;emailId&quot;).value;
	                  
	                  
	                 if(x!= &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; x!=null){ 
	                  /* var atposition=x.indexOf(&quot;@&quot;);  
	                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
	                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
	                    alert(&quot;Please enter valid email id&quot;);  
	                    return false;  
	                    } */
	                   x= x.trim();
	                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
		         			alert(&quot;Please enter valid email id&quot;);
		         			return false;
		         		}
	                 }
	                 		
	                  
	                  

	                  var number12= document.getElementById(&quot;mobileNo&quot;).value;
	                	       number12=number12.trim();
	                	                  if(number12 == &quot;&quot;) {
	                	            	        window.alert(&quot;Mobile number should not be empty&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }

	                	            	    if(number12.length != 10) {
	                	            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
	                	            	        number12.focus();
	                	            	        return false;
	                	            	    }
	                  
	                  
	                  
         				
         					}	
	                  
	                  
         				
         				
         				
         				if( document.getElementById(&quot;fileOrg&quot;).checked )
     					{
         					
        					document.getElementById(&quot;individual&quot;).value=null;
         					document.getElementById(&quot;organisation&quot;).value=&quot;selected&quot;;	
     				
     				 if(document.getElementById(&quot;cusName12&quot;).value == &quot;null&quot; || document.getElementById(&quot;cusName12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                	  {
                	 alert(&quot;Name should not be empty &quot;);
                	 return false;
                	  }
     				 
     				 
     				  var myVar=document.getElementById(&quot;cusName12&quot;).value;
                	  
                	  var hasNumber = /\d/;  
                	  
                	 if(hasNumber.test(myVar))
                		 {
                		 alert(&quot;Name should be alphabate&quot;);
                		 return false;
                		 
                		 }
     				 
     				 
                  
                  if(document.getElementById(&quot;dateOfBirth12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;dateOfBirth12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	 alert(&quot;datofbirth should not be empty &quot;);
            	 return false;
            	  }
                
                  if(document.getElementById(&quot;orgName&quot;).value == &quot;null&quot; || document.getElementById(&quot;orgName&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	 alert(&quot;orgName should not be empty &quot;);
            	 return false;
            	  }
                  var myVar1=document.getElementById(&quot;orgName&quot;).value;
            	  
            	  var hasNumber1 = /\d/;  
            	  
            	 if(hasNumber1.test(myVar1))
            		 {
            		 alert(&quot;Organization Name should be alphabate&quot;);
            		 return false;
            		 
            		 }
                  
                  if(document.getElementById(&quot;mobileNo12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;mobileNo12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	    alert(&quot;mobileNo should not be empty &quot;);
            	    return false;
            	  }
                  
               var number1= document.getElementById(&quot;mobileNo12&quot;).value;
                  
                  number1 = number1.trim();
                  if(number1 == &quot;&quot;) {
            	        window.alert(&quot;Mobile number should not be empty&quot;);
            	        number1.focus();
            	        return false;
            	    }

            	    if(number1.length != 10) {
            	        window.alert(&quot;Phone number must be 10 digits.&quot;);
            	        number1.focus();
            	        return false;
            	    }
               
            	    
            	    var mNumber=document.getElementById(&quot;mobileNo12&quot;).value;
	                  
	                  if(isNaN(mNumber))
	                	  {
	                	  
	                	 alert(&quot;Please enter valid mobile number&quot;);
	                	 return false;
	                	  }
            	    
            	    
            	  
                  /*if(document.getElementById(&quot;emailId12&quot;).value == &quot;null&quot;  || document.getElementById(&quot;emailId12&quot;).value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
            	  {
            	    alert(&quot;emailId should not be empty &quot;);
            	    return false;
            	  }*/
                  
                  
                  
                  
                  
                  
                 var x=document.getElementById(&quot;emailId12&quot;).value;
                 if(x != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; x!=null){ 
                  /* var atposition=x.indexOf(&quot;@&quot;);  
                  var dotposition=x.lastIndexOf(&quot;.&quot;);  
                  if (atposition&lt;1 || dotposition&lt;atposition+2 || dotposition+2>=x.length){  
                    alert(&quot;Please enter valid email id&quot;);  
                    return false;  
                    } */
                    x=x.trim();
                  if (!(/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(x))){
	         			alert(&quot;Please enter valid email id&quot;);
	         			return false;
	         		}
                 }
     					}	
         				
         				var corpdata=document.getElementById(&quot;corporate_1&quot;)
         				
         				var captchaOption = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;optionOfCaptcha&quot; , &quot;'&quot; , &quot;]:checked&quot;).val();
        				document.getElementById(&quot; , &quot;'&quot; , &quot;capchaRadioSel&quot; , &quot;'&quot; , &quot;).value=captchaOption;
         	    		//alert(&quot;captchaOption :: &quot;+captchaOption);
         				
         				var reqDataTmp = &quot; , &quot;'&quot; , &quot;cusName~dateOfBirth~mobileNo~captchaValue~cusName12~dateOfBirth12~mobileNo12~captchaValue12~capchaRadioSel&quot; , &quot;'&quot; , &quot;;//img Aud Captcha
         				//alert(&quot;b4 check empty :&quot;+reqDataTmp);
    					var reqData = checkEmpty(reqDataTmp)
    					//alert(&quot;reqData is &quot;+reqData);
    					var reqDataEmail=document.getElementById(&quot;emailId&quot;)!=null?document.getElementById(&quot;emailId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    				    reqData+=&quot; , &quot;'&quot; , &quot;emailId=&quot; , &quot;'&quot; , &quot;+reqDataEmail+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
    				   // alert(&quot;reqDataEmail is &quot;+reqDataEmail);
    				    var reqDataEmail12=document.getElementById(&quot;emailId12&quot;)!=null?document.getElementById(&quot;emailId12&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    				    reqData+=&quot; , &quot;'&quot; , &quot;emailId12=&quot; , &quot;'&quot; , &quot;+reqDataEmail12+&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;;
    				   //alert(&quot;reqDataEmail12 is &quot;+reqData);
    				    var encdata = getEncryptData(reqData)
    					$(&quot;#encStaticData&quot;).val(encdata);
    				    //alert(&quot;encStaticData is &quot;+encStaticData);

    				 var len = document.getElementById(&quot;paramLen&quot;).value;
    				    
    				  for(i=1,k=11;i&lt;=len;i++,k++)
                     	{ 
                	 var temp;
                	
					if(document.getElementsByName(&quot;outref&quot;+k).length >0){
						
					document.getElementsByName(&quot;outref&quot;+k)[0].value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
					
                	 } 

                }	
               
    				    
    				  /*   var revampcatId=document.getElementById(&quot;categoryId&quot;)!=null?document.getElementById(&quot;categoryId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				var revampInstId=document.getElementById(&quot;instituteId&quot;)!=null?document.getElementById(&quot;instituteId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;  
         				var revampInstutionId=document.getElementById(&quot;institutionId&quot;)!=null?document.getElementById(&quot;institutionId&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampState=document.getElementById(&quot;state&quot;)!=null?document.getElementById(&quot;state&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCategoryName=document.getElementById(&quot;categoryName&quot;)!=null?document.getElementById(&quot;categoryName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampTransactioName=document.getElementById(&quot;transactionName&quot;)!=null?document.getElementById(&quot;transactionName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampcredit=document.getElementById(&quot;creditTxnCount&quot;)!=null?document.getElementById(&quot;creditTxnCount&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampDebit=document.getElementById(&quot;debitBranchCode&quot;)!=null?document.getElementById(&quot;debitBranchCode&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampDob=document.getElementById(&quot;dobName&quot;)!=null?document.getElementById(&quot;dobName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCreditBranch=document.getElementById(&quot;creditBranchCode&quot;)!=null?document.getElementById(&quot;creditBranchCode&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorp=document.getElementById(&quot;corpName&quot;)!=null?document.getElementById(&quot;corpName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorpAdd=document.getElementById(&quot;corpAddress&quot;)!=null?document.getElementById(&quot;corpAddress&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampInst=document.getElementById(&quot;instType&quot;)!=null?document.getElementById(&quot;instType&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampLogopath=document.getElementById(&quot;logoPath&quot;)!=null?document.getElementById(&quot;logoPath&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampLogoName=document.getElementById(&quot;logoName&quot;)!=null?document.getElementById(&quot;logoName&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampIndividual=document.getElementById(&quot;individual&quot;)!=null?document.getElementById(&quot;individual&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCategory=document.getElementById(&quot;Category&quot;)!=null?document.getElementById(&quot;Category&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValid=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampTrans=document.getElementById(&quot;transactionRemarks&quot;)!=null?document.getElementById(&quot;transactionRemarks&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampSelected=document.getElementById(&quot;selectAccountFrom&quot;)!=null?document.getElementById(&quot;selectAccountFrom&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorporateNet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValidNotMand=document.getElementById(&quot;validateNotMandatory&quot;)!=null?document.getElementById(&quot;validateNotMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampValidMand=document.getElementById(&quot;validateMandatory&quot;)!=null?document.getElementById(&quot;validateMandatory&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				var revampCorporateInternet=document.getElementById(&quot;CorporateInternetBanking&quot;)!=null?document.getElementById(&quot;CorporateInternetBanking&quot;).value:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				 */
         				
         				 
         				 
         				  //CR-9184 installment changes start
      	               if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;blank&quot; , &quot;'&quot; , &quot;)
      	            	{
      	                 	alert(&quot;please select payment method&quot;);
      		                return false;
      	                	}
      	                //CR-9184 installment changes end
      					 var val = document.getElementById(&quot;transactionRemarks&quot;).value;
         				 //alert(&quot;Remarks :: &quot;+val);alert(&quot;Remark boolean :: &quot;+val.match(/^[A-Za-z0-9 .,-_/()#\t\r\n\f]+$/i.test($.trim(val)));
         				//alert(&quot;Remark boolean :: &quot;+val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f]*$/)); //^[A-Za-z0-9@_/,&lt;>:^;()# .-]*$
         				if(val != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !(val.match(/^[-_ a-zA-Z0-9./@_,:^;/\()#\t\r\n\f ]*$/)) &amp;&amp; val != null &amp;&amp; val.trim()){
         					alert(&quot;Please enter Remarks in valid format!&quot;);
         					return false;
         				}
         				
 						var encmis = &quot; , &quot;'&quot; , &quot;categoryId~instituteId~institutionId~state~categoryName~dobName~corpName~corpAddress~instType~logoPath~logoName~organisation~individual~Category~transactionRemarks~CorporateInternetBanking~selectAccountFrom~validateMandatory~organisation~formInd~fileOrg&quot; , &quot;'&quot; , &quot;;
         				var reqData = checkEmpty(encmis)
         				var encdata = getEncryptData(reqData)
     					$(&quot;#encMis&quot;).val(encdata);
     				   
     				    
     				   /*  if(document.getElementById(&quot;instType&quot;)!=null)
     				    	{
         				    alert(&quot;before::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	document.getElementById(&quot;instType&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         				    alert(&quot;After::&quot;+document.getElementById(&quot;instType&quot;).value);

     				    	} */
     				    
     				   /*  if(document.getElementById(&quot;validateNotMandatory&quot;)!=null)
     				    	{
     				    	document.getElementById(&quot;validateNotMandatory&quot;)=null;

     				    	//document.getElementById(&quot;validateNotMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
     				    	}
 */     				    
     				  /*  if(document.getElementById(&quot;validateMandatory&quot;)!=null)
				    	{
				    	document.getElementById(&quot;validateMandatory&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	} */
     				    
     				  /* if(document.getElementById(&quot;CorporateInternetBanking&quot;)!=null)
				    	{
				    	document.getElementById(&quot;CorporateInternetBanking&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	}  
				    	
				  	  if(document.getElementById(&quot;selectAccountFrom&quot;)!=null)
				    	{
				    	document.getElementById(&quot;selectAccountFrom&quot;).value=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
				    	}  */

     				//alert(&quot;Hi &quot;+document.getElementById(&quot;validateNotMandatory&quot;).value);alert(&quot;2 &quot;+document.getElementById(&quot;organisation&quot;).value);
     				
     				   //removed from here
   		         
   		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
    				    
    				    
    				    
    				    
         				
         				if(!corpdata.checked)
         					{
         					alert(&quot;Please read and accept the Terms and Conditions&quot;);
         					return false;
         					}
    				
         				var capt=document.getElementById(&quot;captchaValue&quot;).value
         				
         				if(capt == &quot;null&quot; || capt == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
         					{
         					
         					alert(&quot;Please enter captcha value as shown in the image or from the audio&quot;);
         					return false;
         					}
         				else
         					{
         				//	alert(&quot;Am here!!&quot;);
         				//	alert(capt);
         					//	
         					var alphaNumExp = new RegExp(&quot;^[0-9a-zA-Z]+$&quot;);
         					 if(!document.getElementById(&quot;captchaValue&quot;).value.match(alphaNumExp)) {
								alert(&quot; , &quot;'&quot; , &quot;Please enter valid text as shown in the image or from the audio&quot; , &quot;'&quot; , &quot;);
								document.getElementById(&quot;captchaValue&quot;).value=&quot;&quot;;
								document.getElementById(&quot;captchaValue&quot;).focus();
								return false;
						      }else{
         							validateCaptcha(&quot; , &quot;'&quot; , &quot;paymentDetails&quot; , &quot;'&quot; , &quot;,capt);
         					  }
         					}
         				
     	                
         				// $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);   
       				    // $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         				  
         				  
         				 //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
      	              //   $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
      	               
         				  
         				  //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		             // $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
      		            //document.getElementById(&quot;validateNotMandatory&quot;)=null;
      		             //alert(&quot;data is &quot;+document.getElementById(&quot;validateMandatory&quot;));
      		             
      		             
      		             	//CR-9184 installment changes start
					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;partial&quot; , &quot;'&quot; , &quot;)
					{
					   //alert(&quot;one&quot;);
						$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;installmentdetails.htm&quot; , &quot;'&quot; , &quot;);
						}
					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;full&quot; , &quot;'&quot; , &quot;))
						{
						//alert(&quot;two&quot;);
						$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;installmentdetailsconfirmpage.htm&quot; , &quot;'&quot; , &quot;);
						}
					//CR-9184 installment changes end
					else
					   {
						//alert(&quot;three&quot;);
					$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;confirmpayment.htm&quot; , &quot;'&quot; , &quot;);
					 }
				
       				//$(&quot; , &quot;'&quot; , &quot;#paymentDetails&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;confirmpayment.htm&quot; , &quot;'&quot; , &quot;);
        				
         				
         }
 
         	function validateCaptcha(formName, captchaValue) {
         		//img Aud Captcha
        		var captchaOption = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;optionOfCaptcha&quot; , &quot;'&quot; , &quot;]:checked&quot;).val();
        		document.getElementById(&quot; , &quot;'&quot; , &quot;capchaRadioSel&quot; , &quot;'&quot; , &quot;).value=captchaOption;
         		
         	      var captcha = $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + captchaValue).val();
         	      
         	     captcha=captchaValue;
         	      
         	      var validateurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/validatecaptcha.htm&quot; , &quot;'&quot; , &quot;;
         	     
         	    var reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/simpleCaptchaServ?&quot; , &quot;'&quot; , &quot;; //img Aud Captcha 
         	      var errorurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/sessiontimeout.htm&quot; , &quot;'&quot; , &quot; ; 
         	    
         	       var successurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/confirmpayment.htm&quot; , &quot;'&quot; , &quot; ;
         	      
         	       var installmentDetails = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/installmentdetails.htm&quot; , &quot;'&quot; , &quot; ;
         	     
         	       var installmentDetailsConfirm = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/installmentdetailsconfirmpage.htm&quot; , &quot;'&quot; , &quot; ;
         	    //audio
         			if(captchaOption ==&quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;)
         				reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/audio.wav?bogus=&quot; , &quot;'&quot; , &quot;;
         	      
         	      $.ajax({
         	            type : &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
         	            dataType : &quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;,
         	            url : validateurl,
         	          //  data : &quot; , &quot;'&quot; , &quot;captchaVal=&quot; , &quot;'&quot; , &quot; + captcha,//commented for img Aud Captcha
		    				data:jQuery.param({captchaVal:captcha,captchaOption:captchaOption}),
         	            success : function(data) { 
         	            	
         	            	
         	            	
         	                  if (data == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;) {
         	                	  
         	                	 $(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#installmentParamKey&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#netbankingflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#branchflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#creditcardflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	              		 $(&quot; , &quot;'&quot; , &quot;#otherdebitflag&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); $(&quot; , &quot;'&quot; , &quot;#installmentParamKey&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	              		
         	              		 $(&quot; , &quot;'&quot; , &quot;#controls&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         	                	  
         	                	 $(&quot; , &quot;'&quot; , &quot;#paramLen&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);   
        				    	 $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        				    	 $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	               $(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	                //$(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	               // $(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
             	                //$(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
             	                $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);  
             	               $(&quot; , &quot;'&quot; , &quot;#organisation&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
             				   $(&quot; , &quot;'&quot; , &quot;#categoryId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#instituteId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#institutionId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#state&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#categoryName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#transactionName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#creditTxnCount&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#debitBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#dobName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               //$(&quot; , &quot;'&quot; , &quot;#creditBranchCode&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        			
        		               //$(&quot; , &quot;'&quot; , &quot;#corpName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#corpAddress&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#instType&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#logoPath&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#logoName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
          			
           		               $(&quot; , &quot;'&quot; , &quot;#individual&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#Category&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		              // $(&quot; , &quot;'&quot; , &quot;#selectAccountFrom&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        			
        		               //$(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		              // $(&quot; , &quot;'&quot; , &quot;#validateNotMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               //$(&quot; , &quot;'&quot; , &quot;#validateMandatory&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		              // $(&quot; , &quot;'&quot; , &quot;#CorporateInternetBanking&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
          			
           		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#cusName12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#dateOfBirth12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
        		               $(&quot; , &quot;'&quot; , &quot;#capchaRadioSel&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);//imag and audio captcha
        		               $(&quot; , &quot;'&quot; , &quot;#mobileNo12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#captchaValue12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#emailId12&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#formInd&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
           		               $(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);  
           		               $(&quot; , &quot;'&quot; , &quot;#corporate_1&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;);
         	                	  
         	                 	//CR-9184 installment changes start
         	 					if(document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;partial&quot; , &quot;'&quot; , &quot;)
         	 					{
         	 						$(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
      	                              action : installmentDetails
      	                        	});
         	 						}
         	 					else if((document.getElementById(&quot;installment&quot;)!=null &amp;&amp; document.getElementById(&quot;installment&quot;).value==&quot; , &quot;'&quot; , &quot;full&quot; , &quot;'&quot; , &quot;))
         	 						{
         	 						 $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
        	                              action : installmentDetailsConfirm
        	                        });
         	 						 }
         	 					//CR-9184 installment changes end
         	 					else
         	 					   {
         	 						 $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
        	                              action : successurl
        	                        }); 
         	 						 
         	 					   }
         	 				
         	                 	
         	                    //   alert(&quot;data in captcha&quot;+document.getElementById(&quot;validateMandatory&quot;));

         	                    //  document.getElementById(&quot;validateMandatory&quot;)=null;
         	                    //alert(&quot;1111111111111111111111111111111111111111&quot;);
         	                       $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).submit();
         	                      
         	                  }else if(data ==&quot; , &quot;'&quot; , &quot;sessiontimeout&quot; , &quot;'&quot; , &quot;){
         	                        $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).attr({
         	                              action : errorurl
         	                        });   
         	                                                
         	                        $(&quot; , &quot;'&quot; , &quot;form#paymentDetails&quot; , &quot;'&quot; , &quot;).submit();
         	                  }else {

         	                        alert(&quot; , &quot;'&quot; , &quot;Please enter valid captcha as shown in the image or from the audio&quot; , &quot;'&quot; , &quot;);
         	                      //  $(&quot; , &quot;'&quot; , &quot;#captcha&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
         	                      $(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
         	                       document.getElementById(&quot;captchaValue&quot;).value = &quot;&quot;;
         	                        document.getElementById(&quot;imageContainer&quot;).removeChild(
         	                    //    document.getElementById(&quot;captchaImage&quot;));
         	                        		document.getElementById(&quot;refreshImgCaptcha&quot;));//img Aud Captcha
         	                        var newImg = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
         	                      //  newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;captchaImage&quot;);
         	                      newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;refreshImgCaptcha&quot;);//img Aud Captch
         	                        newImg.setAttribute(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, reloadurl+ new Date().getTime());
         	                        newImg.setAttribute(&quot; , &quot;'&quot; , &quot;alt&quot; , &quot;'&quot; , &quot;, &quot;Captcha&quot;);
         	                        var div = document.getElementById(&quot; , &quot;'&quot; , &quot;imageContainer&quot; , &quot;'&quot; , &quot;);
         	                        div.appendChild(newImg);

         	                  }
         	            },
         	            error : function(data) {
         	                  alert(&quot; , &quot;'&quot; , &quot;error : &quot; , &quot;'&quot; , &quot; + data);
         	            }
         	     
         	      }); 
         	    // document.getElementById(&quot;validateMandatory&quot;)=null;
         	      
         	      
         	}  

     
         	function captchaRefresh()
         	{
         		$(&quot; , &quot;'&quot; , &quot;#captcha&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
                 document.getElementById(&quot;imageContainer&quot;).removeChild(
                             document.getElementById(&quot;captchaImage&quot;)); 
                             var reloadurl = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/loadcaptcha.htm?&quot; , &quot;'&quot; , &quot;;
                 var newImg = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;captchaImage&quot;);
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, reloadurl+ new Date().getTime());
                 newImg.setAttribute(&quot; , &quot;'&quot; , &quot;alt&quot; , &quot;'&quot; , &quot;, &quot;Captcha&quot;);
                 var div = document.getElementById(&quot; , &quot;'&quot; , &quot;imageContainer&quot; , &quot;'&quot; , &quot;);
                 div.appendChild(newImg);
         	}
         	
         	
         	
         	
         	function backInstitution()	{
         		document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		document.paymentDetails.submit();
         	}

         	function resetFeeParams(formName)	{

         		if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateMandatory&quot; , &quot;'&quot; , &quot;) != null){
         			var validateMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;);		
         			if(validateMandatory.length==undefined)
         				validateMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateMandatory&quot; , &quot;'&quot; , &quot;));
         				
         			for(i=0;i&lt;validateMandatory.length;i++){
         				
         				var validations=validateMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         				var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validations[0]);
         				
         				if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		  }

         		if(document.getElementById(&quot; , &quot;'&quot; , &quot;validateNotMandatory&quot; , &quot;'&quot; , &quot;) != null)	{
         			
         			var validateNotMandatory=eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;);		
         			if(validateNotMandatory.length==undefined)
         				validateNotMandatory = new Array(eval(formName+&quot; , &quot;'&quot; , &quot;.validateNotMandatory&quot; , &quot;'&quot; , &quot;));
         				
         			for(i=0;i&lt;validateNotMandatory.length;i++){
         				
         				var validationsNotMand =validateNotMandatory[i].value.split(&quot; , &quot;'&quot; , &quot;#SEP#&quot; , &quot;'&quot; , &quot;);
         				var controlObj = eval(formName+&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;+validationsNotMand[0]);
         				
         				if(controlObj.type.indexOf(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;) != -1){				
         					controlObj.selectedIndex=0;		
         				}			
         				else if(controlObj.readOnly!=true){
         				var etx = controlObj.readOnly;
         					controlObj.value=&quot;&quot;;
         				}
         			}
         		} 
         		
         		$(&quot; , &quot;'&quot; , &quot;#cusName&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#dateOfBirth&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#mobileNo&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#emailId&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#captchaValue&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);$(&quot; , &quot;'&quot; , &quot;#transactionRemarks&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
         		
         	}
         	
         	
         	function OpenExcel(path,name)
         	{
         	   //var wind=window.open(path+name,&quot;Report&quot;,&quot;toolbar=yes,scrollbars=yes,resizable=yes,top=0,left=0,menuBar=yes&quot;);
         		path=path+name;//alert(&quot;full : &quot;+path);
        		var link=document.createElement(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);
        		link.href = path;
        		link.download = path.substr(path.lastIndexOf(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;) + 1);
        		link.click();
         	}

         	
         	function getEncryptData(reqData)
         	{
         		var key = $(&quot;#eKeVal&quot;).val();
         		if(key != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
         		{
         			
         			return CryptoJS.AES.encrypt(reqData,key)
         		}
         	}

         	
         	function checkEmpty(reqData)
         	{
         		var validInput = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
         		if(reqData.indexOf(&quot;~&quot;) > 0)
         		{
         			var arr = reqData.split(&quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot;);
         			for(var i=0;i&lt;arr.length;i++)
         			{ 
         				var temp;
         				if(document.getElementById(arr[i]) !=null)
         				 temp = document.getElementById(arr[i]).value;
         				if(temp == &quot;&quot;) {
         					validInput += arr[i]+&quot;=&quot;+&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;+&quot;&amp;&quot;;
         				}else {
         					validInput += arr[i]+&quot;=&quot;+temp+&quot;&amp;&quot;;
         				}
         			}
         			return validInput;
         		}
         	
         	
 
         
         
         	}  
         	
         	
         	function myFunction1()
         	{
         		
     			 var w = document.paymentDetails.Category.selectedIndex;
    			 var categoryId1=&quot; , &quot;'&quot; , &quot;C592823&quot; , &quot;'&quot; , &quot;;
    			 var ins1=&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;;
    			 var st1=&quot; , &quot;'&quot; , &quot;Tamil Nadu&quot; , &quot;'&quot; , &quot;;
    	
    			 
    			document.getElementById(&quot;categoryId&quot;).value=categoryId1;
    		    document.getElementById(&quot;institutionId&quot;).value=ins1;
    			document.getElementById(&quot;state&quot;).value=st1;
    	
    	
    	var selected_text = document.paymentDetails.Category.options[w].text;
    	
    	if (selected_text == &quot;--Select Category--&quot;) {
    		 
    		
    		 return false;
    		 
    	} else {
    		
    		document.paymentDetails.action=&quot;showpaymentdetails.htm&quot;;
    		
    		document.paymentDetails.categoryId.value =category.options[category.selectedIndex].value;
    	
    		
    		var fileMode=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;fileMode&quot; , &quot;'&quot; , &quot;);
    	
    		var paramName=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;paramName&quot; , &quot;'&quot; , &quot;);
    		
    		var optStr=category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;optString&quot; , &quot;'&quot; , &quot;);
    	
    		var dupPayment = category.options[category.selectedIndex].getAttribute(&quot; , &quot;'&quot; , &quot;dupPayment&quot; , &quot;'&quot; , &quot;);
    		
             //API    
    		if(fileMode==&quot; , &quot;'&quot; , &quot;Y&quot; , &quot;'&quot; , &quot; || fileMode==&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;)
    		{
    			
    		
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;; 
    				var arr = paramName.split(&quot;-&quot;);
    				$(&quot;#numberofkeyfield&quot;).val(arr.length); 
    				
    				
    				
    				
    				document.getElementById(&quot;fileMode&quot;).value=&quot;Y&quot;; 
    				
    				var arr = paramName.split(&quot;-&quot;);
        			
        			         				
    				document.getElementById(&quot;firstkey&quot;).value=arr[0]; 
    				document.getElementById(&quot;secondkey&quot;).value=arr[1]; 
    				
    				
    				
    				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/filemode.htm&quot; , &quot;'&quot; , &quot;;
    				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				

    			
    			
    			}
    		else
    			{
    			
    				
    				document.getElementById(&quot;categoryNameSel&quot;).value=&quot;categorySelected&quot;;  
    				
    				
    				var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listcategory.htm&quot; , &quot;'&quot; , &quot;;
    				
    				
    				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
    				$(&quot;#paymentDetails&quot;).submit();
    				
    				
    				return false; 
    			}
    		return true;
    		} 
    }
         	
         	
         	function restFeeParams(formName)	{
         		
         		var indOrOrgVal = &quot;&quot;; //def7 
         		if( $(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ){ //def7 
         			indOrOrgVal = &quot;fileOrg&quot;;
         		} 
         		
         	
         			 var selectedCaptcha = $(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha]:checked&quot; , &quot;'&quot; , &quot;).val();  //def 9
         	
         			
         		
          		 document.getElementById(&quot; , &quot;'&quot; , &quot;loginAudioCaptcha&quot; , &quot;'&quot; , &quot;).controls = false; //def 8
         		 document.getElementById(&quot; , &quot;'&quot; , &quot;paymentDetails&quot; , &quot;'&quot; , &quot;).reset();
         		document.getElementById(&quot; , &quot;'&quot; , &quot;loginAudioCaptcha&quot; , &quot;'&quot; , &quot;).controls = true; //def 8
         		// getUserSelImgCaptcha();
         		 
         		 if(indOrOrgVal == &quot; , &quot;'&quot; , &quot;fileOrg&quot; , &quot;'&quot; , &quot;){ //def7 
         			$(&quot; , &quot;'&quot; , &quot;#fileOrg&quot; , &quot;'&quot; , &quot;).prop( &quot;checked&quot;, true );
         		 }else{
         			$(&quot; , &quot;'&quot; , &quot;#formInd&quot; , &quot;'&quot; , &quot;).prop( &quot;checked&quot;, true );
         		 }//def7 
         		 
         		
         		 if(selectedCaptcha == &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;){  //def 9
         			$(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha][value=AUD]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
         			getUserSelAudCaptcha();
         		 }else{
          			$(&quot; , &quot;'&quot; , &quot;input[name=optionOfCaptcha][value=IMG]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
         			getUserSelImgCaptcha();
            		 
         		 } //def 9
         		}
         	function backFeeParams(formName)	{
         		
         		 
         		var insName=&quot; , &quot;'&quot; , &quot;Educational&quot; , &quot;'&quot; , &quot;;
         		//var insID =&quot; , &quot;'&quot; , &quot;IN502269&quot; , &quot;'&quot; , &quot;
         		document.getElementById(&quot; , &quot;'&quot; , &quot;instType&quot; , &quot;'&quot; , &quot;).value=insName;
         		//document.getElementById(&quot; , &quot;'&quot; , &quot;institutionId&quot; , &quot;'&quot; , &quot;).value=insID;
         		//document.paymentDetails.action=&quot;listinstitution.htm&quot;;
         		//document.paymentDetails.submit();
         		//Added by supriya for back button issue
         		var url = &quot;/sbicollect&quot;+&quot; , &quot;'&quot; , &quot;/payment/listinstitution.htm&quot; , &quot;'&quot; , &quot;;
				
				
				$(&quot;#paymentDetails&quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
				$(&quot;#paymentDetails&quot;).submit();
				//Added by supriya for back button issue
         	}
         	
         	 //Function to set the image captcha as default option on selection of individula/org radio button
         	function resetToImageCaptcha(){
         		if($(&quot; , &quot;'&quot; , &quot;input:radio[name=optionOfCaptcha]:checked&quot; , &quot;'&quot; , &quot;).val()==&quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;)
         	   {
         	   	$(&quot; , &quot;'&quot; , &quot;input:radio[name=optionOfCaptcha]&quot; , &quot;'&quot; , &quot;)[0].checked = true; 
               	getUserSelImgCaptcha();
         	   }
         	}
         	
         
	  




		
		
		
			
		
		
		 
        
            
                 © State Bank of India 
            
            
                Privacy Statement 
                 
                Disclosures
				 
				Terms of Use
            
        
		  		
		 
   
  
   $(document).ready(function() {
   var filename = window.location.href.substr(window.location.href.lastIndexOf(&quot;/&quot;)+1);
   
     switch(filename) {
	  case &quot;icollecthome.htm&quot;:   
	 $(&quot;#Link_1&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
    case &quot;paymenthistory.htm&quot;:   
	 $(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
    case &quot;faq.htm&quot;:    
    $(&quot;#Link_3&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
     break;
	 case &quot;customersupport.htm&quot;:    
   $(&quot;#Link_4&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    break;
	 case &quot;paymenthistorydatedetails.htm&quot;:
	$(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
	break;
	 case &quot;sendotp.htm&quot;:
	$(&quot;#Link_2&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
	break;
  default:
   $(&quot;#Link_1&quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);    
  } 
   
});


   
		
		
		
	
	


 
	


	




   	
 
	
  
      
       




			
				
					
					
					
					
					Thank you for choosing SB Collect. As per RBI guidelines on cross-border payment transactions, maximum amount of Rs 25,00,000/- per transaction / per item can only be processed on this platform. Hence, this transaction is declined. On clicking &quot;OK&quot; you will be redirected to SB Collect Corporate home page. Inconvenience regretted.
					
					
					
					OK
					
				
			



function logout()
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;logout.htm&quot; , &quot;'&quot; , &quot;);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}

function callURL(url)
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}
function callLogout(url)
{
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}

/* function callMopsPage() {
	
	var domain=document.domain;
	var win = window.open(&quot; , &quot;'&quot; , &quot;https://&quot; , &quot;'&quot; , &quot;+domain+&quot; , &quot;'&quot; , &quot;/prelogin/mopsremittanceform.htm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);

//	var win = window.open(&quot; , &quot;'&quot; , &quot;https://www.onlinesbi.com/prelogin/mopsremittanceform.htm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);

	if (win) {
	    win.focus();
	}
} */
function callInst(url)
{
	  // alert(&quot; , &quot;'&quot; , &quot;callInst&quot; , &quot;'&quot; , &quot;+url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#logoutform&quot; , &quot;'&quot; , &quot;).submit();
}
function callCat(url){
//	alert(&quot; , &quot;'&quot; , &quot;callact&quot; , &quot;'&quot; , &quot;);
	$(&quot; , &quot;'&quot; , &quot;#institutionform&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, url);
	$(&quot; , &quot;'&quot; , &quot;#institutionform&quot; , &quot;'&quot; , &quot;).submit();
	
}
 function paySubmit(payflag)
	{
		//alert(&quot; , &quot;'&quot; , &quot;payflag&quot; , &quot;'&quot; , &quot;+payflag);
		var hourCheck = $(&quot; , &quot;'&quot; , &quot;#hourCheck&quot; , &quot;'&quot; , &quot;).val();
		var minCheck = $(&quot; , &quot;'&quot; , &quot;#minCheck&quot; , &quot;'&quot; , &quot;).val();
		
		var foreignCardTxnLimit = parseFloat(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		var amountTransfer      = parseFloat(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		
		if(((hourCheck==23 &amp;&amp; minCheck>=30) || (hourCheck==00 &amp;&amp; minCheck&lt;=30) ) &amp;&amp; (payflag==&quot; , &quot;'&quot; , &quot;SBDEBIT&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;OTHERDCARD&quot; , &quot;'&quot; , &quot;||payflag==&quot; , &quot;'&quot; , &quot;CREDITCARD&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;PREPAID&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;RUPAYCARD&quot; , &quot;'&quot; , &quot;))
		{
		 	alert(&quot;This payment mode is not available between 23:30 hours IST and 00:30 hours IST&quot;);
		}else if(((hourCheck == 22 &amp;&amp; minCheck >= 30) || (hourCheck == 23 &amp;&amp; minCheck &lt;= 30) ) &amp;&amp; (payflag == &quot; , &quot;'&quot; , &quot;UPI&quot; , &quot;'&quot; , &quot;))
		{
		 	alert(&quot;This payment mode is not available between 22:30 hours IST and 23:30 hours IST&quot;);
		} else {
			
			$(&quot; , &quot;'&quot; , &quot;#payflag&quot; , &quot;'&quot; , &quot;).val(payflag);

			if( payflag==&quot; , &quot;'&quot; , &quot;SBDEBIT&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;OTHERDCARD&quot; , &quot;'&quot; , &quot;||payflag==&quot; , &quot;'&quot; , &quot;CREDITCARD&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;PREPAID&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; ||payflag==&quot; , &quot;'&quot; , &quot;RUPAYCARD&quot; , &quot;'&quot; , &quot;) {
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fsspaymentgateway.htm&quot; , &quot;'&quot; , &quot;);
			}else if(payflag == &quot; , &quot;'&quot; , &quot;UPI&quot; , &quot;'&quot; , &quot;){
			$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;upipayment.htm&quot; , &quot;'&quot; , &quot;);
			
		
			}else {
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;suvidhapayment.htm&quot; , &quot;'&quot; , &quot;);
			}
			
			if(payflag==&quot; , &quot;'&quot; , &quot;FOREIGN&quot; , &quot;'&quot; , &quot; &amp;&amp; amountTransfer > foreignCardTxnLimit){
				$(&quot; , &quot;'&quot; , &quot;#transactionModal&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
			}else{
				$(&quot; , &quot;'&quot; , &quot;#frmFeeParams&quot; , &quot;'&quot; , &quot;).submit();
			}		
		}
	}
 function submitVPA()
 { //alert(&quot; , &quot;'&quot; , &quot;submitVPA&quot; , &quot;'&quot; , &quot;);
 	/* var validator = $(&quot;#frmupi&quot;).validate (
 	{
 		focusCleanup: true,
 		onkeyup: false,
 		onblur: false,
 		onfocusout:false,
 		rules:{
 			vpa:{ required : true,
 				vpacheck : true}
 		},
 		messages: {
 			vpa :{
 				required :&quot;Please enter VPA&quot;,
 				vpacheck:&quot;You have entered an invalid VPA!&quot;
 				}
 		}
 	}); */
 	
 /* 	if (validator.form()) { */
 	//	doencrypt();
 		$(&quot; , &quot;'&quot; , &quot;form#frmupi&quot; , &quot;'&quot; , &quot;).attr({
 			action : &quot; , &quot;'&quot; , &quot;validateVPA.htm&quot; , &quot;'&quot; , &quot;
 		});	
 		$(&quot; , &quot;'&quot; , &quot;#frmupi&quot; , &quot;'&quot; , &quot;).submit();
 	//}
 }




$(document).ready(function () {
	
	//Disable full page
/*   $(&quot;body&quot;).on(&quot;contextmenu&quot;,function(e) {
      return false;
  }); */
  
  $(this).bind(&quot;contextmenu&quot;, function(e) {
      e.preventDefault();
  });
	
  function disableBack() { window.history.forward() }
  
  window.onload = disableBack();
  window.onpageshow = function(evt) { if (evt.persisted) disableBack() }

    
  $(document).keydown(function (e) {
        return (e.which || e.keyCode) != 116;
  });
    
});




    $(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).select2();



/html[1]/body[1]/section[@class=&quot;section_div&quot;]/div[@class=&quot;col-lg-12  col-md-12 col-sm-12 col-12 content_section&quot;]&quot;))]</value>
      <webElementGuid>8ee600c3-a565-4533-a2af-7798f0abca79</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
