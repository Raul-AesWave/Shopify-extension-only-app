import {
  reactExtension,
  BlockStack,
  Checkbox,
  Text,
  TextField,
  Link,
  useAppMetafields,
  useBuyerJourneyIntercept,
  useExtensionCapability,
} from '@shopify/ui-extensions-react/checkout';
import { useState } from 'react';
import { useApplyAttributeChange, useApplyNoteChange } from '@shopify/ui-extensions-react/checkout';

export default reactExtension("purchase.checkout.block.render", () => <Extension />);

function Extension() {
  const metafields = useAppMetafields({
    type: 'shop',
    namespace: 'custom',
    key: 'tos_pdf_link',
  });

  const [showNotes, setShowNotes] = useState(false);
  const [note, setNote] = useState('');

  const canBlockProgress = useExtensionCapability("block_progress")
  const [agreed, setAgreed] = useState(false); 
  const [validationError, setValidationError] = useState(null); 
  const clearValidationErrors = () => setValidationError(null);

  const tosLink = metafields[0]?.metafield?.value || 'https://storage.googleapis.com/resources.aeswave.com/TOS/TOS%202012%20v1.0.pdf';

  useBuyerJourneyIntercept(({ canBlockProgress }) => {
    if (!agreed && canBlockProgress) {
      return {
        behavior: 'block',
        reason: 'You must agree to the Terms of Service before completing your order.',
        perform: (result) => {
          if (result.behavior === 'block') {
            setValidationError('Agree to Terms of Service')
          }
        },
        errors: [
          {
            message: 'Must Agree to Terms of Service.',        
          }
        ]
      };
    }
    return { 
      behavior: 'allow',
      perform: () => {
        clearValidationErrors();
      }
     };
  });

const applyAttributeChange = useApplyAttributeChange();
const applyNoteChange = useApplyNoteChange();

const handleChange = async (checked) => {
  setAgreed(checked);
  clearValidationErrors();

  const timestamp = new Date().toISOString();

  if (checked) {
    await applyAttributeChange({
      key: 'tos_agreed_time',
      type: 'updateAttribute',
      value: timestamp,
    });

    await applyAttributeChange({
      key: 'tos_link_at_agreement',
      type: 'updateAttribute',
      value: tosLink,
    });

    await applyAttributeChange({
      key: 'agree_to_terms',
      type: 'updateAttribute',
      value: 'yes',
    });
  } else {
    await applyAttributeChange({
      key: 'agree_to_terms',
      type: 'updateAttribute',
      value: 'no',
    });

    await applyAttributeChange({
      key: 'tos_agreed_time',
      type: 'updateAttribute',
      value: '',
    });

    await applyAttributeChange({
      key: 'tos_link_at_agreement',
      type: 'updateAttribute',
      value: '',
    });
  }
}


  return (
    <BlockStack spacing="loose">
      {/* Start Order Notes */}
      <Checkbox
        id="show-notes"
        name="show-notes"
        checked={showNotes}
        onChange={(checked) => {
          setShowNotes(checked);
        }}
      >
        Add Order Notes
      </Checkbox>

      {showNotes && (
        <TextField
          label="Order Notes"
          multiline
          value={note}
          onChange={(value) => {
            setNote(value);
            applyNoteChange({ type: 'updateNote', note: value }); // ✅ Save to Shopify Order Note field
          }}
        />
      )}
      {/* End Order Notes */}
      {/* Start TOS */}
      <Text size="large">Terms of Service Agreement</Text>

      <Checkbox
        id="tos-agree"
        name="tos-agree"
        required={canBlockProgress}
        onChange={handleChange}
        error= {validationError}
        checked={agreed}
        onInput = {clearValidationErrors}
      >
        I agree to the Terms of Service
      </Checkbox>

      <Link to={tosLink} target="_blank">
        View full Terms of Service (PDF)
      </Link>
      {/* Start TOS */}
    </BlockStack>
  );
}
