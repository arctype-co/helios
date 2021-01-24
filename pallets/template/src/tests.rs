use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, traits::Vec, dispatch::DispatchError};
use frame_system::{RawOrigin};

#[test]
fn test_transaction_ids() {
	new_test_ext().execute_with(|| {
        let data1 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0a];
        let data2 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0b, 0x05];
        let data3 = vec![0x0b, 0x0e, 0x0e, 0x0f];
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::transact(Origin::signed(1), 0, 0, data1));
		assert_ok!(TemplateModule::transact(Origin::signed(1), 0, 0, data2));
		assert_ok!(TemplateModule::transact(Origin::signed(2), 0, 0, data3));
        assert_eq!(TemplateModule::last_transaction_id(), 3);
	});
}

#[test]
fn test_empty_transaction_fails() {
	new_test_ext().execute_with(|| {
        let empty_data = Vec::new();
        assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 0, empty_data),
            Error::<Test>::EmptyTransaction
        );
	});
}

#[test]
fn test_max_transaction_size() {
	new_test_ext().execute_with(|| {
        let data1 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0a];
        let data2 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0b, 0x05];
        let data3 = vec![0x0b, 0x0e, 0x0e, 0x0f];
        assert_noop!(
            TemplateModule::admin_set_max_transaction_size(Origin::signed(1), 8),
            DispatchError::BadOrigin
        );
        assert_ok!(TemplateModule::admin_set_max_transaction_size(RawOrigin::Root.into(), 4));
		assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 0, data1),
            Error::<Test>::TransactionOverflow
        );
            
		assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 0, data2),
            Error::<Test>::TransactionOverflow
        );
		assert_ok!(TemplateModule::transact(Origin::signed(2), 0, 0, data3));
        assert_eq!(TemplateModule::last_transaction_id(), 1);
	});
}

#[test]
fn test_transaction_keys() {
	new_test_ext().execute_with(|| {
        let data1 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0a];
        let data2 = vec![0x0b, 0x00, 0x00, 0x0b, 0x0b, 0x05];
        let data3 = vec![0x0b, 0x0e, 0x0e, 0x0f];
        let data4 = vec![0x0b, 0x0e, 0x0e, 0x0f];
        let data5 = vec![0x0b, 0x0e, 0x0e, 0x0f];
		assert_ok!(TemplateModule::transact(Origin::signed(1), 0, 0, data1));
		assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 1, data2),
            Error::<Test>::InvalidKeyId
        );
        assert_ok!(TemplateModule::admin_enable_key(RawOrigin::Root.into(), 1));
		assert_ok!(TemplateModule::transact(Origin::signed(1), 0, 1, data3));
        assert_ok!(TemplateModule::admin_disable_key(RawOrigin::Root.into(), 1));
		assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 1, data4),
            Error::<Test>::InvalidKeyId
        );
        assert_ok!(TemplateModule::admin_disable_key(RawOrigin::Root.into(), 0));
		assert_noop!(
            TemplateModule::transact(Origin::signed(1), 0, 0, data5),
            Error::<Test>::InvalidKeyId
        );
	});
}
