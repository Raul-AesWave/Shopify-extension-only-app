import {
  reactExtension,
  BlockStack,
  Checkbox,
  Text,
  Link,
  useAppMetafields,
  useBuyerJourney,
} from '@shopify/ui-extensions-react/checkout';
// change to 'purchase.checkout.actions.render-before' to render for the final pay button in checkout can only be used in prod
export default reactExtension('purchase.checkout.block.render', () => <Extension />);

function Extension() {
  const metafields = useAppMetafields({
    type: 'shop',
    namespace: 'custom',
    key: 'tos_pdf_link',
  });

  const { setCanContinue } = useBuyerJourney();

  const tosLink = metafields[0]?.metafield?.value || 'https://storage.googleapis.com/resources.aeswave.com/TOS/TOS%202012%20v1.0.pdf';

  const handleChange = (checked) => {
    setCanContinue(checked);
  };

  return (
    <BlockStack spacing="loose">
      <Text size="large">Terms of Service Agreement</Text>

      <Checkbox
        id="tos-agree"
        name="tos-agree"
        checked={false}
        onChange={handleChange}
      >
        I agree to the Terms of Service
      </Checkbox>

      <Link to={tosLink} target="_blank">
        View full Terms of Service (PDF)
      </Link>
    </BlockStack>
  );
}
